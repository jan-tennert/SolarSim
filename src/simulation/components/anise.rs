use std::str::FromStr;
use anise::constants::frames::{EARTH_MOON_BARYCENTER_J2000, SUN_J2000, VENUS_J2000};
use anise::prelude::{Almanac, Epoch, SPK};

pub fn load_ephemeris(
    path: &str,
) {
    let spk = SPK::load(path).unwrap();
    let ctx = Almanac::from_spk(spk).unwrap();
    let epoch = Epoch::from_str("2023-10-01 00:00:00.0000 TDB").unwrap();

    let state = ctx
        .translate(
            VENUS_J2000, // Target
            SUN_J2000, // Observer
            epoch,
            None,
        )
        .unwrap();
    println!("{}", state);
}