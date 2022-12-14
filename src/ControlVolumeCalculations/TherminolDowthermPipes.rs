#![warn(missing_docs)]
extern crate uom;
use uom::si::available_energy::joule_per_kilogram;
use uom::si::f64::*;
use uom::si::*;
use uom::si::heat_capacity::joule_per_kelvin;
use crate::ControlVolumeCalculations::ExplictCalculations::*;
use crate::ControlVolumeCalculations::
FluidEntity_StructsAndTraits::*;


// here are imports for units
use uom::si::time::second;
use uom::si::volume::cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;

extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::therminol_component::
FluidProperties;
use fluid_mechanics_rust::therminol_component::
dowtherm_a_properties;

#[derive(Clone)]
/// This struct or class represents a fixed heat
/// flux therminol pipe
/// 
/// Here, we don't consider conjugate heat transfer 
/// or anything, but we just supply a fixed heat value to
/// the therminol pipe
///
/// ```rust
/// 
/// extern crate approx;
/// use heat_transfer_rust::ControlVolumeCalculations::
/// TherminolDowthermPipes::*;
///
/// use uom::si::f64::*;
/// use uom::si::time::second;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use uom::si::volume::cubic_meter;
///
/// // first let's initialise a pipe using a timestep
/// // global thermodynamic temperature and fluid volume
/// 
/// let timestep = Time::new::<second>(0.1_f64);
/// let initial_global_temp = ThermodynamicTemperature::
/// new::<kelvin>(300_f64);
///
/// let fluid_volume = Volume::new::<cubic_meter>(
/// 0.01_f64.powf(3_f64));
///
/// let fluid_entity_index: i32 = 4;
///
/// // we are now going to initialise stuff
///
/// use heat_transfer_rust::ControlVolumeCalculations::
/// FluidEntity_StructsAndTraits::FluidEntityInitialisationSteps;
///
/// let mut pipe1 = FixedHeatFluxTherminolPipe::new();
///
/// pipe1.step_0_set_timestep_and_initial_temperatures(
/// timestep,
/// initial_global_temp,
/// fluid_volume,
/// fluid_entity_index);
///
///
///
/// ```
///
pub struct FixedHeatFluxTherminolPipe {
    pub fluid_parameters: FluidEntityThermophysicalData,
}

impl FixedHeatFluxTherminolPipe {

    pub fn new() -> Self {

        // we'll need to make a few data structures first
        // with some default values


        let default_timestep: Time = 
            Time::new::<second>(0.1);

        let default_fluid_volume: Volume = 
            Volume::new::<cubic_meter>(0.02_f64.powf(3.0));

        // let's populate default index data
        
        let default_index: i32 = 0;

        let default_index_data = 
            FluidEntityIndexData { 
                fluid_entity_index : default_index ,
                inlet_fluid_entity_index : default_index,
                outlet_fluid_entity_index : default_index,
            };


        // let's populate default temperature data
        //

        let default_temperature : ThermodynamicTemperature 
            = ThermodynamicTemperature::new::
            <kelvin>(310_f64);

        let default_temperature_data : PipeFluidTemperatureData  
            = PipeFluidTemperatureData { 
                inlet_temp_old: default_temperature, 
                inlet_temp_new: default_temperature, 
                outlet_temp_old: default_temperature, 
                outlet_temp_new: default_temperature, 
                fluid_temp_old: default_temperature, 
                fluid_temp_new: default_temperature,
            };

        // let's finally populate the enthalpy data

        let default_enthalpy : AvailableEnergy =
            AvailableEnergy::new::<joule_per_kilogram>(0_f64);

        let default_enthalpy_data : PipeFluidEnthalpyData =
            PipeFluidEnthalpyData { 
                inlet_enthalpy_old: default_enthalpy, 
                inlet_enthalpy_new: default_enthalpy, 
                outlet_enthalpy_old: default_enthalpy, 
                outlet_enthalpy_new: default_enthalpy, 
                fluid_enthalpy_old: default_enthalpy, 
                fluid_enthalpy_new: default_enthalpy 
            };

        let default_thermophysical_data : FluidEntityThermophysicalData
            = FluidEntityThermophysicalData { 
                index_data: default_index_data, 
                temperature_data: default_temperature_data, 
                enthalpy_data: default_enthalpy_data, 
                timestep: default_timestep, 
                fluid_volume: default_fluid_volume ,
            };

        return Self { 
            fluid_parameters: default_thermophysical_data 
        };

    }
}

impl FluidEntityInitialisationSteps for FixedHeatFluxTherminolPipe {
    /// step zero, essentially the constructor

    fn step_0_set_timestep_and_initial_temperatures(
        &mut self,
        timestep: Time,
        initial_global_temp: ThermodynamicTemperature,
        fluid_volume: Volume,
        fluid_entity_index: i32) -> Self {

        FluidEntityThermophysicalData::
            step_0_set_timestep_and_initial_temperatures(
                &mut self.fluid_parameters, 
                timestep, 
                initial_global_temp, 
                fluid_volume, 
                fluid_entity_index
                );

        return self.clone();
    }
    

    fn step_1_connect_to_component_inlet(
        &mut self,
        other_fluid_entity: &mut FluidEntityThermophysicalData) {

        FluidEntityThermophysicalData::
            step_1_connect_to_component_inlet(
                &mut self.fluid_parameters, 
                other_fluid_entity);
    }

    fn step_2_conenct_to_component_outlet(
         &mut self,
         other_fluid_entity: &mut FluidEntityThermophysicalData){

        FluidEntityThermophysicalData::
            step_2_conenct_to_component_outlet(
                &mut self.fluid_parameters, 
                other_fluid_entity);

    }

    fn step_3_add_component_to_vector(
        &mut self,
        fluid_entity_vector: &mut Vec<FluidEntityThermophysicalData>){

        FluidEntityThermophysicalData::
            step_3_add_component_to_vector(
                &mut self.fluid_parameters, 
                fluid_entity_vector);
    }
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

/// Contains functions which return the
/// viscosity, density, enthalpy, specific heat capacity
/// and thermal conductivity for therminol VP 1 or
/// dowtherm A in the range 20-180C
///
/// The dowtherm A correlations are used
/// 
pub trait TherminolFluidProperties {

    /// returns dowtherm A density given a temperature
    fn density(fluid_temp: ThermodynamicTemperature) 
        -> MassDensity {
        return dowtherm_a_properties::getDowthermADensity(fluid_temp);
    }

    /// returns dowtherm A dynamic viscosity given
    /// a temperature
    fn viscosity(
        fluid_temp: ThermodynamicTemperature) -> DynamicViscosity{
        return dowtherm_a_properties::getDowthermAViscosity(fluid_temp);
    }

    ///returns dowtherm A specific
    ///enthalpy  given a temperature
    ///
    /// 0 J/kg specific enthalpy is assumed at 20C
    /// and everything is calculated from there
    fn enthalpy(fluid_temp: ThermodynamicTemperature) -> AvailableEnergy{
        return dowtherm_a_properties::getDowthermAEnthalpy(fluid_temp);
    }

    /// returns dowtherm A specific heat capacity
    ///
    fn specific_heat_capacity(
        fluid_temp: ThermodynamicTemperature) -> SpecificHeatCapacity{
        return dowtherm_a_properties::
            getDowthermAConstantPressureSpecificHeatCapacity(
            fluid_temp);
    }

    /// returns dowtherm A thermal conductivity
    fn thermal_conductivity(
        fluid_temp: ThermodynamicTemperature) -> ThermalConductivity{
        return dowtherm_a_properties::
            getDowthermAThermalConductivity(fluid_temp);
    }

    ///returns dowtherm A temperature
    ///  given a specific enthalpy
    ///
    /// 0 J/kg specific enthalpy is assumed at 20C
    /// and everything is calculated from there
    fn get_temperature_from_enthalpy(
        fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature{
        return dowtherm_a_properties::
            get_temperature_from_enthalpy(fluid_enthalpy);
    }

}

