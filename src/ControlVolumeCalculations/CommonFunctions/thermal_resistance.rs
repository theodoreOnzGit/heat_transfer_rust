//! this module  contains functions for thermal resistance
//!
//!
//! will probably need to cite later, [to be done] 
//! take fourier's law for example
//!
//! heat flux = - k dT/dx
//! Q/A  = -k dT/dx
//!
//! the thermal resistance here is the ratio of
//! the driving force (dT) to the heat flow (Q)
//!
//! in non differential form,
//! Q/A = -k (Delta T)/(Delta x)
//! 
//! -(Delta T)/Q = (Delta x)/(kA)
//!
//! and for convection
//!
//! heat flux (surface to fluid) = - h (T_fluid - T_surface)
//! 
//! Q/A = - (Delta T) h
//!
//! (Delta T)/Q = 1/(hA)
//!
//! The unit for thermal resistance here is kelvin per watt
//! 
//! For all intents and purposes however, 
//! we want to find the heat transfer given a set temperature difference
//! and properties of the pipe and etc
//!
//! hence, the output of the functions here will usually be power
//! given various inputs
//!
//!
use uom::si::f64::*;
use uom::si::power::watt;

/// calcualtes heat flow using a thermal resistance model,
/// Q/A = - (Delta T) h
///
/// thermal resistance is:
/// (Delta T)/Q = 1/(hA)
pub fn obtain_power_through_single_convection_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_surface_area: Area,
    heat_transfer_coefficient: HeatTransfer,
    ) -> Power {


    // thermal resistance
    //
    

    let thermal_resistance = 
        1.0_f64
        /average_surface_area
        /heat_transfer_coefficient;

    use super::*;
    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;
}

/// calcualtes heat flow using a thermal resistance model,
/// -(Delta T)/Q = (Delta x)/(kA)
///
/// assumes there are two layers in the 1D system
pub fn obtain_power_through_two_layer_wall_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_thermal_conductivity_layer_1: ThermalConductivity,
    average_thermal_conductivity_layer_2: ThermalConductivity,
    average_surface_area_1: Area,
    average_surface_area_2: Area,
    length_of_wall_1: Length,
    length_of_wall_2: Length) -> Power {

    // (Delta x)/(kA) for first layer
    let thermal_resistance_1 = 
        length_of_wall_1
        /average_thermal_conductivity_layer_1
        /average_surface_area_1;

    // (Delta x)/(kA) for first layer
    let thermal_resistance_2 = 
        length_of_wall_2
        /average_thermal_conductivity_layer_2
        /average_surface_area_2;

    let thermal_resistance = 
        thermal_resistance_1 
        + thermal_resistance_2;

    use super::*;
    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;


}

/// calcualtes heat flow using a thermal resistance model,
/// -(Delta T)/Q = (Delta x)/(kA)
pub fn obtain_power_through_wall_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_thermal_conductivity: ThermalConductivity,
    average_surface_area: Area,
    length_of_wall: Length) -> Power {

    // (Delta x)/(kA)
    let thermal_resistance = 
        length_of_wall
        /average_thermal_conductivity
        /average_surface_area;

    use super::*;
    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;


}



