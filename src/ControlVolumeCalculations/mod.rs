//! this module contains functions to calculate the enthalpy
//! of a control volume at the the next timestep
//!
//!
//! For Control Volume calculations in general
//! we have the form:
//!
//! dH_cv/dt = H_in - H_out + Q_s + W_s
//!
//! H_cv is the control volume enthalpy
//!
//! H_in is the sum of enthalpy flows in
//!
//! H_out is the sum of enthalpy flows out
//!
//! Q_s is the heat supplied to the volume per second
//!
//! W_s is the work done on the system per second
//!
//!
//! After discretisation, we can use:
//!
//! (H_cv (t+1) - H_cv (t)) / dt = H_in - H_out + Q_s + W_s
//!
//! H_cv (t+1)  = dt * (H_in - H_out + Q_s + W_s) + H_cv (t)
//!
//! It remains to be seen whether the enthalpy flows in and
//! out are calculated at the current time step  (explicit)
//! or next time step (implict)
//!
//! Of course, implicit calculations are more stable but
//! slower in general than explicit calculations
//!
//! we will be using the uom module to ensure that calculations are
//! done with correct units



extern crate uom;
use uom::si::f64::*;
pub mod ExplictCalculations;

pub fn get_control_volume_enthalpy_next_timestep(
    timestep: Time,
    enthalpy_out: Power,
    enthalpy_in: Power,
    heat_supplied_to_system: Power,
    work_done_on_system: Power,
    control_volume_enthalpy_current_timestep: Energy,
    ) -> Energy {

    let control_volume_enthalpy_next_timestep: Energy 
        = timestep * (enthalpy_in - 
                      enthalpy_out +
                      heat_supplied_to_system +
                      work_done_on_system) 
        + control_volume_enthalpy_current_timestep;

    return control_volume_enthalpy_next_timestep;

}
