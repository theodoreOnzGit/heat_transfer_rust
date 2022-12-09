extern crate uom;
use uom::si::f64::*;


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
