# Simulation

![image](https://github.com/jan-tennert/Simulation/assets/26686035/45124abd-f053-4b41-9bf9-16a9db9f7ae7)

A complete simulation of our solar system made with [Bevy](https://bevyengine.org/) with many features including:

- True to scale gravitational simulation of all bodies

- Simulated planetary rotation speeds

- Drawing orbital lines of bodies

- Manipulating orbits by changing masses

- High quality 3D models of all planets and moons from NASA

- Speed up the simulation by increasing substeps (higher accuracy, slower) or timesteps (decreases accuracy, no difference in speed)

- Built-in scenario editor to create and save your own scenarios.
  - Support for loading ephemeris and rotation data from SPICE files \
    *Note that the SPICE files themselves are not included in the binary and repository due to their size*

### Installation

Download the newest binaries from the [releases](https://github.com/jan-tennert/Simulation) and run the binary to get started!

### Scenarios

You can load, edit and create scenarios within the application. A scenario is a collection of bodies with their initial positions, velocities, rotations and other properties.
![image](https://github.com/user-attachments/assets/faa0cb63-1bac-4f7d-a341-e363d290ac41)

SPICE files can be obtained from [NASA](https://naif.jpl.nasa.gov/naif/) and some also from the Rust Toolkit used, [ANISE](https://github.com/nyx-space/anise). 
They should be stored in the `data` folder. If loading for the first time, SolarSim copies the SPICE files to the `data` folder.
- If you load in a SPK file like `de400s.bsp`, you can load in starting positions and velocities by putting in the `Ephemeris ID` in the body panel and clicking `Load starting data`.
- If you load in a PCA file like `pck11.pca`, you can load in rotation data and shape data by putting in the fixed frame ids (target id/observer id) in the body panel and clicking `Load starting data`.

### Sponsoring

If you like the project and want to support it, consider sponsoring me on [Ko-fi](https://ko-fi.com/jantennert) or directly on [PayPal](https://www.paypal.com/donate/?hosted_button_id=SR3YJS5CZFS9L).
