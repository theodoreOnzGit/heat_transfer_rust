extern crate uom;
use uom::si::f64::*;
/// This function calculates the formula:
/// H_cv (t+1)  = dt * (H_in - H_out + Q_s + W_s) + H_cv (t)
///
/// It is up to user discretion whether to use the enthalpy
/// in and out of the system and work done on the system
/// as well as heat supplied at current timestep or next timestep
///
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

/// calculates enthalpy flow from
/// mass flowrate and specific energy (or available energy)
///
/// I will need to also calculate specific energy given a temperature.
/// This is fluid specific and we will need to code
/// correlations to convert specific energy (or available energy)
/// to temperature and vice versa
pub fn calculate_enthalpy_flow(
    m : MassRate,
    h : AvailableEnergy) -> Power {

    // m is mass flowrate
    // h is enthalpy per unit mass
    //
    // we are returning m * h which is enthalpy flowrate

    return m * h ;
}

/// calculate convection heat flux power input
///
/// Q = h (T_surface - T_fluid) A
pub fn calculate_convection_heat_flux_power_input(
    h : HeatTransfer,
    T_surface : TemperatureInterval,
    T_fluid : TemperatureInterval,
    A : Area) -> Power {

    return h * (T_surface - T_fluid) * A; 

}


/// calculate overall heat flux power input
///
/// Q = U (T_surrounding - T_fluid) A
pub fn calculate_overall_heat_flux_power_input(
    U : HeatTransfer,
    T_surrounding : TemperatureInterval,
    T_fluid : TemperatureInterval,
    A : Area) -> Power {

    return U * (T_surrounding - T_fluid) * A; 

}

