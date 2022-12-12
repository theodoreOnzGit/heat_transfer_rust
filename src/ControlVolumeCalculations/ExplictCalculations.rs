extern crate uom;
use uom::si::f64::*;

/// For explicit calculations in general
///
/// we use the equation:
///
/// h_new = h_old + deltaT * (H_in - H_out +
/// Q + W)
///
/// What we need here:
///
/// we take in the current T_in and T_out of
/// the pipe or thermal component
///
/// old and new as well
///
/// and T_sys old and new
///
/// To get enthalpy, I will then need to convert
/// these temperatures to enthalpy
///
///
pub struct PipeFluidTemperatureData {
    inlet_temp_old: ThermodynamicTemperature,
    inlet_temp_new: ThermodynamicTemperature,
    outlet_temp_old: ThermodynamicTemperature,
    outlet_temp_new: ThermodynamicTemperature,
    fluid_temp_old: ThermodynamicTemperature,
    fluid_temp_new: ThermodynamicTemperature
}

pub struct TherminolPipeFluidEnthalpyData {
    temperature_data: PipeFluidTemperatureData,

    inlet_enthalpy_old: AvailableEnergy,
    inlet_enthalpy_new: AvailableEnergy,
    outlet_enthalpy_old: AvailableEnergy,
    outlet_enthalpy_new: AvailableEnergy,
    fluid_enthalpy_old: AvailableEnergy,
    fluid_enthalpy_new: AvailableEnergy
}
