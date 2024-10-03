use std::error::Error;
use crate::simulation::asset::serialization::SerializedVec;
use bevy::utils::HashMap;
use reqwest::blocking::Client;
use std::str::FromStr;
use bevy::math::DVec3;
use bevy::prelude::{Component, Plugin, Query, Res, ResMut, Resource};
use chrono::NaiveDateTime;
use crate::simulation::scenario::setup::ScenarioData;
use crate::simulation::components::selection::SelectedEntity;
use crate::simulation::ui::editor_body_panel::EditorPanelState;
use crate::simulation::ui::toast::{error_toast, success_toast, ToastContainer};

pub struct HorizonsPlugin;

impl Plugin for HorizonsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<HorizonsClient>();
    }
}

#[derive(Resource)]
pub struct HorizonsClient(pub Client);

impl Default for HorizonsClient {
    fn default() -> Self {
        Self(Client::new())
    }
}

#[derive(Component, Clone, Default)]
pub struct NaifIdComponent(pub i32);

const HORIZONS_API_URL: &'static str = "https://ssd.jpl.nasa.gov/api/horizons.api?format=text";

pub struct HorizonsApiParameters {

    params: HashMap<String, String>

}

impl HorizonsApiParameters {

    pub fn with_defaults() -> Self {
        let mut params = HashMap::new();
        params.insert("CENTER".to_string(), "500@0".to_string()); //Solar System Barycenter
        params.insert("MAKE_EPHEM".to_string(), "YES".to_string());
        params.insert("EPHEM_TYPE".to_string(), "VECTORS".to_string());
        params.insert("CSV_FORMAT".to_string(), "YES".to_string());
        Self { params }
    }

    pub fn with_command(mut self, command: i32) -> Self {
        self.params.insert("COMMAND".to_string(), command.to_string());
        self
    }

    pub fn with_start_time(mut self, start_time: &str) -> Self {
        self.params.insert("START_TIME".to_string(), start_time.to_string());
        self
    }

    pub fn with_stop_time(mut self, stop_time: &str) -> Self {
        self.params.insert("STOP_TIME".to_string(), stop_time.to_string());
        self
    }

    pub fn with_center(mut self, center: i32) -> Self {
        self.params.insert("CENTER".to_string(), center.to_string());
        self
    }

}

pub fn get_starting_data_horizons(
    parameters: HorizonsApiParameters,
    client: Client
) -> Result<(SerializedVec, SerializedVec), Box<dyn Error>> {
    let mut builder = client.get(HORIZONS_API_URL);

    // Add query parameters to the request
    for (key, value) in parameters.params.iter() {
        builder = builder.query(&[(&key, &value)]);
    }

    // Send the request and handle potential errors
    let resp = builder.send()?;

    // Get the response body as text and handle potential errors
    let body = resp.text()?;

    // Process the lines of the body and find the data row
    let lines = body.lines();
    let mut iter = lines.skip_while(|line| !line.starts_with("$$SOE"));

    // Skip the "$$SOE" line and move to the data line, returning an error if the data line is missing
    iter.next(); // Skip the "$$SOE" line
    let data_line = iter.next().ok_or("No data row found after $$SOE")?;

    // Split the data row and map each value, handling errors during parsing
    let data_row = data_line
        .split(", ")
        .skip(2)
        .map(|d| d.trim())
        .collect::<Vec<&str>>();

    if data_row.len() < 6 {
        return Err("Data row does not contain enough values".into());
    }

    // Parse the individual values into f64, returning an error if parsing fails
    let vec1 = SerializedVec {
        x: f64::from_str(data_row[0])?,
        y: f64::from_str(data_row[1])?,
        z: f64::from_str(data_row[2])?,
    };
    let vec2 = SerializedVec {
        x: f64::from_str(data_row[3])?,
        y: f64::from_str(data_row[4])?,
        z: f64::from_str(data_row[5])?,
    };

    Ok((vec1, vec2))
}

pub fn retrieve_starting_data_horizons(
    selected_entity: Res<SelectedEntity>,
    bodies: Query<&NaifIdComponent>,
    client: Res<HorizonsClient>,
    mut state: ResMut<EditorPanelState>,
    scenario: Res<ScenarioData>,
    mut toasts: ResMut<ToastContainer>
) {
    if let Some(entity) = selected_entity.entity {
        let starting_date = NaiveDateTime::from_timestamp_millis(scenario.starting_time_millis).unwrap().date();
        let start_date = starting_date.format("%Y-%m-%d").to_string();
        let stop_date = (starting_date + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        let id = bodies.get(entity).unwrap();
        let parameters = HorizonsApiParameters::with_defaults()
            .with_command(id.0)
            .with_start_time(start_date.as_str())
            .with_stop_time(stop_date.as_str());
        if let Ok((pos, vel)) = get_starting_data_horizons(parameters, client.0.clone()) {
            state.new_position = DVec3::from(pos);
            state.new_velocity = DVec3::from(vel);
            toasts.0.add(success_toast("Horizons data retrieved"));
        } else {
            toasts.0.add(error_toast("Failed to retrieve Horizons data. Check the Horizons Id and try again."));
        }
    }
}