//! in this module, I want to house traits for control volumes
//!
//! a control volume in the heat transfer context will just
//! be a fixed region of space.
//!
//! This region will have fluid flowing in and out of the control volume
//! thus bringing enthalpy in and out of the control volume
//!
//! also there is heat and work just like the
//! first law of thermodynamics
//!
//! the idea here is to calculate the enthalpy of
//! the control volume at the next timestep so to speak
//!
//! the thing here is that the control volume is perfectly well
//! mixed so that the outlet temperature is equal to the bulk temperature
//! of the control volume
//!

use uom::si::f64::*;
use super::CommonFunctions::*;
pub trait ControlVolume {}

/// contains associated functions for the control
/// volume,
///
/// not meant for making into trait objects
pub trait ControlVolumeAssociatedFunctions {

    /// calculates enthalpy of a control volume at the next timestep
    ///
    /// this assumes that the mass of the control volume does not change
    /// in any significant way
    fn calculate_specific_enthalpy_at_next_timestep(
        current_timestep_control_volume_specific_enthalpy: AvailableEnergy,
        current_timestep_control_volume_mass: Mass,
        timestep: Time,
        enthalpy_out: Power,
        enthalpy_in: Power,
        heat_supplied_to_system: Power,
        work_done_on_system: Power,
        ) -> Result<AvailableEnergy,AvailableEnergy> {

        let control_volume_enthalpy_current_timestep: Energy =
            current_timestep_control_volume_mass
            * current_timestep_control_volume_specific_enthalpy;

        let control_volume_enthalpy_next_timestep = 
            get_control_volume_enthalpy_next_timestep(
                timestep, 
                enthalpy_out, 
                enthalpy_in, 
                heat_supplied_to_system, 
                work_done_on_system, 
                control_volume_enthalpy_current_timestep);

        // we are assuming control volume mass does not change
        let specific_enthalpy_next_timestep = 
            control_volume_enthalpy_next_timestep/
            current_timestep_control_volume_mass;

        return Ok(specific_enthalpy_next_timestep);
    }

    

}
