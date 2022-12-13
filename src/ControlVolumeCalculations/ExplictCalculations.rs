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
/// (1) To get enthalpy, I will then need to convert
/// these temperatures to enthalpy, so i will need methods
/// for that
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
    inlet_enthalpy_old: AvailableEnergy,
    inlet_enthalpy_new: AvailableEnergy,
    outlet_enthalpy_old: AvailableEnergy,
    outlet_enthalpy_new: AvailableEnergy,
    fluid_enthalpy_old: AvailableEnergy,
    fluid_enthalpy_new: AvailableEnergy,

}

/// This structure stores the index
/// of the fluid entity (pipe or some other component)
/// 
/// as well as the indices of the pipes or fluid entities
/// connected to the inlet and outlet
pub struct FluidEntityIndexData {
    fluid_entity_index: i32,
    inlet_fluid_entity_index: i32,
    outlet_fluid_entity_index: i32,
}



/// This trait helps the developer run through the steps
/// of enthalpy calculation
///
///
pub trait ExplicitCalculationSteps {


    /// First Step: calculate enthalpies and bulk fluid temp
    /// from temperatures
    fn step_1_calculate_current_timestep_temp_enthalpies(
        &mut self);

    /// Second Step: calculate new system enthalpy from available
    /// enthalpies, heat loss/gain and work done rates
    /// will probably require timestep also
    fn step_2_calculate_new_system_enthalpy(
        &mut self, 
        heat_supplied_to_fluid: Power,
        work_done_on_fluid: Power,
        timestep: Time,
        fluid_mass_flowrate: MassRate);

    /// third step: calculate new system
    /// temperature based on new system enthalpy
    ///
    /// we will then obtain the inlet and outlet
    /// temperatures based on the system temperatures
    /// of each pipe using some matrix solver
    /// which should give us a vector of inlet
    /// temperatures
    fn step_3_calculate_new_system_temperature(
        &mut self) -> ThermodynamicTemperature;

    /// after finding the vector of inlet temperatures
    /// we should either give the thermodynamic temperautre
    /// and put it into the function
    /// 
    /// or make a copy of the vector of inlet temperature
    /// and feed it to this struct so that it can edit 
    /// its own data
    fn step_4_set_inlet_temperature(
        &mut self,
        new_inlet_temperature: ThermodynamicTemperature);

    /// after finding the vector of inlet temperatures
    /// we can next map the inlet temperatures to the
    /// proper outlet temperature vector and also feed it
    /// in
    fn step_5_set_inlet_temperature(
        &mut self,
        new_outlet_temperature: ThermodynamicTemperature);


    /// now that we have all the required information,
    /// we can set the old temperatures to the values of
    /// the new temperatures
    fn step_6_update_current_timestep_temperatures(
        &mut self);
}


pub trait FluidEntityInitialisationSteps {

    /// Step zero: set timestep and initial temperautres
    ///
    /// Also, the fluid volume for the fluid portion of the
    /// pipe can be assumed fixed (in this case we ignore 
    /// thermal expansion for simplicity)
    /// Otherwise, fluid volume and fluid density must be
    /// taken at each timestep as appropriate parameters
    fn step_0_set_timestep_and_initial_temperatures(
        &mut self,
        timestep: Time,
        initial_global_temp: ThermodynamicTemperature,
        fluid_volume: Volume);

    /// Step 1: connect a pipe or some other structure
    /// to the inlet to this component or fluid entity
    fn step_1_connect_component_inlet(
        &mut self,
        other_fluid_entity: &mut impl FluidEntityInitialisationSteps);

    /// Step 2: connect a pipe or some other structure
    /// to the outlet of this component or fluid entity
    ///
    /// This step is optional because step 1 should be
    /// able to connect pipe A's inlet to pipe B's outlet
    fn step_2_conenct_component_outlet(
        &mut self,
        other_fluid_entity: &mut impl FluidEntityInitialisationSteps);

    /// Step 3: add component to list or vector of components
    fn step_3_add_component_to_vector(
        &mut self,
        fluid_entity_vector: &mut Vec<impl FluidEntityInitialisationSteps>
        );

}

extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::therminol_component::
FluidProperties;
use fluid_mechanics_rust::therminol_component::
dowtherm_a_properties;


impl FluidProperties for TherminolPipeFluidEnthalpyData {

    fn density(fluid_temp: ThermodynamicTemperature) -> MassDensity {
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















