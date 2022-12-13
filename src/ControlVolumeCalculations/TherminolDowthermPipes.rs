#![warn(missing_docs)]
extern crate uom;
use uom::si::f64::*;
use uom::si::*;
use crate::ControlVolumeCalculations::ExplictCalculations::*;
use crate::ControlVolumeCalculations::
FluidEntity_StructsAndTraits::*;

extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::therminol_component::
FluidProperties;
use fluid_mechanics_rust::therminol_component::
dowtherm_a_properties;

pub struct FixedHeatFluxTherminolPipe {
    pub fluid_parameters: FluidEntityThermophysicalData,
}


impl ExplicitCalculationSteps for FixedHeatFluxTherminolPipe {
    fn step_1_calculate_current_timestep_temp_enthalpies(
        &mut self) {
        // first let's get the temperatures
        //
        let current_inlet_temp = 
            self.fluid_parameters.temperature_data.
            inlet_temp_old.clone();

        let current_outlet_temp = 
            self.fluid_parameters.temperature_data.
            outlet_temp_old.clone();

        // now i want to find the average temperature
        // of these temperatures
        let current_bulk_temp_value_kelvin = 
            (current_inlet_temp.value 
             + current_outlet_temp.value)/2.0;

        let current_bulk_temp = 
            ThermodynamicTemperature::new::<
            thermodynamic_temperature::kelvin>
            (current_bulk_temp_value_kelvin);

        self.fluid_parameters.temperature_data.
            fluid_temp_old = 
            current_bulk_temp.clone();

        // with these values let's find the enthalpies
        self.fluid_parameters.enthalpy_data.
            inlet_enthalpy_old = 
            FixedHeatFluxTherminolPipe::
            enthalpy(current_inlet_temp);

        self.fluid_parameters.enthalpy_data.
            outlet_enthalpy_old = 
            FixedHeatFluxTherminolPipe::
            enthalpy(current_outlet_temp);

        self.fluid_parameters.enthalpy_data.
            fluid_enthalpy_old =
            FixedHeatFluxTherminolPipe::
            enthalpy(current_bulk_temp);
    }

    /// we are calculating
    ///
    /// h_new = h_old + delta T * 
    /// (H_in - H_out + Q + W)
    fn step_2_calculate_new_system_enthalpy(
        &mut self, 
        heat_supplied_to_fluid: Power,
        work_done_on_fluid: Power,
        timestep: Time,
        fluid_mass_flowrate: MassRate){

        // first let's calculate H_in

        let enthalpy_flowrate_in = 
            fluid_mass_flowrate *
            self.fluid_parameters.
            enthalpy_data.inlet_enthalpy_old.clone();
            
        // second let's calculate H_out

        let enthalpy_flowrate_out = 
            fluid_mass_flowrate *
            self.fluid_parameters.
            enthalpy_data.outlet_enthalpy_old.clone();

        // third let's calculate heat addition

        let heat_addition : Energy = 
            timestep *
            (enthalpy_flowrate_in - enthalpy_flowrate_out 
             + heat_supplied_to_fluid 
             + work_done_on_fluid);

        // fourth, let's calculate the new system enthalpy
        // for this, we need the current of the control
        // volume times specific enthalpy

        let control_volume_mass : Mass = 
            FixedHeatFluxTherminolPipe::density(
                self.fluid_parameters.temperature_data.
                fluid_temp_old)*
            self.fluid_parameters.fluid_volume;

        let h_old : Energy = 
            control_volume_mass *
            self.fluid_parameters.enthalpy_data.
            fluid_enthalpy_old;

        let h_new : Energy = 
            h_old + heat_addition;

        // fifth, assuming mass does not change much
        // we can find the new system specifc enthalpy

        let new_fluid_specifc_enthalpy : AvailableEnergy 
            = h_new/control_volume_mass;

        self.fluid_parameters.enthalpy_data.
            fluid_enthalpy_new = new_fluid_specifc_enthalpy;

        // and that's it!
    }

    /// This step calculates the new system temperature
    /// from system specific enthalpy estimates
    fn step_3_calculate_new_system_temperature(
        &mut self) -> ThermodynamicTemperature {

        let new_fluid_temp_average : ThermodynamicTemperature
            = FixedHeatFluxTherminolPipe::
            get_temperature_from_enthalpy(
                self.fluid_parameters.enthalpy_data.
                fluid_enthalpy_new);

        self.fluid_parameters.temperature_data.
            fluid_temp_new = 
            new_fluid_temp_average.clone();

        return new_fluid_temp_average;
    }

    /// This step assumes that the new inlet temperature
    /// has been calculated, and sets the new inlet
    /// temperature as such
    fn step_4_set_inlet_temperature(
        &mut self,
        new_inlet_temperature : ThermodynamicTemperature) {

        self.fluid_parameters.temperature_data.
            inlet_temp_new = new_inlet_temperature;

    }

    /// This step assumes that the new outlet temperature
    /// has been calculated, and sets the new outlet
    /// temperature as such
    fn step_5_set_outlet_temperature(
        &mut self,
        new_outlet_temperature : ThermodynamicTemperature) {

        self.fluid_parameters.temperature_data.
            outlet_temp_new = new_outlet_temperature;

    }


    /// Assuming all new temperatures have been calculated
    /// one can update set the old temperatures
    /// to the new ones at the next timestep
    fn step_6_update_current_timestep_temperatures(
        &mut self) {

        self.fluid_parameters.temperature_data.
            inlet_temp_old =
            self.fluid_parameters.temperature_data.
            inlet_temp_new.clone();

        self.fluid_parameters.temperature_data.
            outlet_temp_old =
            self.fluid_parameters.temperature_data.
            outlet_temp_new.clone();

        self.fluid_parameters.temperature_data.
            fluid_temp_old =
            self.fluid_parameters.temperature_data.
            fluid_temp_new.clone();
    }
}

impl TherminolFluidProperties for FixedHeatFluxTherminolPipe {
}

pub trait TherminolFluidProperties {
    fn density(fluid_temp: ThermodynamicTemperature) 
        -> MassDensity {
        return dowtherm_a_properties::getDowthermADensity(fluid_temp);
    }

    fn viscosity(
        fluid_temp: ThermodynamicTemperature) -> DynamicViscosity{
        return dowtherm_a_properties::getDowthermAViscosity(fluid_temp);
    }

    fn enthalpy(fluid_temp: ThermodynamicTemperature) -> AvailableEnergy{
        return dowtherm_a_properties::getDowthermAEnthalpy(fluid_temp);
    }

    fn specific_heat_capacity(
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity{
        return dowtherm_a_properties::
            getDowthermAConstantPressureSpecificHeatCapacity(
            fluid_temp);
    }

    fn thermal_conductivity(
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity{
        return dowtherm_a_properties::
            getDowthermAThermalConductivity(fluid_temp);
    }

    fn get_temperature_from_enthalpy(
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature{
        return dowtherm_a_properties::
            get_temperature_from_enthalpy(fluid_enthalpy);
    }

}

