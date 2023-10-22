# Simulation

https://github.com/jan-tennert/Simulation/assets/26686035/0345b4f1-479d-4888-be3a-5652cec5d38d

A complete simulation of our solar system made with [Bevy](https://bevyengine.org/) with many features including:

- True to scale gravitational simulation of all bodies

- Simulated planetary rotation speeds

- Drawing orbital lines of bodies

- Manipulating orbits by changing masses

- High quality 3D models of all planets and moons from NASA

- Speed up the simulation by increasing substeps or timesteps

- External editor "horizon-ui" to add, edit and remove bodies

### Installation

Download the newest binaries from the [releases](https://github.com/jan-tennert/Simulation) and run `Simulation.exe` to get started!

### Adding new bodies

If you want to customize or even add new bodies, run `horizon-ui.jar`. You can right click on stars and planets to add children. 

Make sure you pay attention to the units!

You can get vector positions and velocity on the [Horizons System](https://ssd.jpl.nasa.gov/horizons/app.html#/) website. You have to make sure you have the right settings:

- For the Ephemeris Type, select Vector Table

- For the Target Body, you can search for the body you want to add.

- You have to make sure that all bodies were measured at the same time, so to make  sure: 
  
  - Open the Horizon Ui
  
  - Click on Options and select Change Time Settings
  
  - There should be the date all bodies should be from
  
  So if you have the date, click on Time Specification and change the starting date depending on what you got from horizon-ui

- The coordinate center & table settings should remain as is.

Then click on **Generate Ephemeris** and read the first entry of the list.

![image](https://github.com/jan-tennert/Simulation/assets/26686035/26858c7c-9ac3-4438-8794-b590f897c0f4)

The first row will be your starting positions and the second row your starting velocity, you can paste them as is into the HorizonUi.


