extern crate uom;
use ndarray::FixedInitializer;
use uom::si::f64::*;
use uom::si::mass_rate::kilogram_per_second;
use crate::ControlVolumeCalculations::Sandbox::*;
use uom::si::time::second;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::power::watt;

pub struct FluidEntityCollectionV1 {

    pub current_max_index: usize,
    pub fluid_entity_vector: Vec<v2_IterativeHeatFluxTherminolPipe>,
    pub inlet_temp_vec: Vec<ThermodynamicTemperature>,
    pub outlet_temp_vec: Vec<ThermodynamicTemperature>,

    pub heat_input_vec: Vec<Power>,
    pub work_input_vec: Vec<Power>,
    pub mass_flowrate_vec: Vec<MassRate>,

    pub timestep: Time,
    pub initial_global_temp: ThermodynamicTemperature,

}

use crate::ControlVolumeCalculations::FluidEntity_StructsAndTraits::*;

impl FluidEntityCollectionV1 {

    /// default constructor
    /// sets timestep at 0.1s by default
    pub fn new() -> Self {

        return Self { 
            current_max_index: 0, 
            fluid_entity_vector: vec![], 
            inlet_temp_vec: vec![], 
            outlet_temp_vec: vec![], 
            heat_input_vec: vec![], 
            work_input_vec: vec![], 
            mass_flowrate_vec: vec![], 

            /// Default timestep is 0.1s
            timestep: Time::new::<second>(0.1),
            /// Default initial global temp is 300k or about 27C
            initial_global_temp: ThermodynamicTemperature::new::
                <kelvin>(300.0),
        }

    }

    pub fn step_0_set_timestep_and_initial_temp(
        &mut self,
        timestep: Time,
        initial_global_temp: ThermodynamicTemperature) {

        self.timestep = timestep;
        self.initial_global_temp = initial_global_temp;
    }



    /// Adds a new fluid component or fluid entity
    pub fn step_1_add_new_component(
        &mut self,
        fluid_volume: Volume
        ) {

        let mut new_pipe = v2_IterativeHeatFluxTherminolPipe::new();

        let fluid_entity_index = self.current_max_index;

        // we make a new fluid entity
        new_pipe.step_0_set_timestep_and_initial_temperatures(
            self.timestep,
            self.initial_global_temp,
            fluid_volume,
            fluid_entity_index);

        // i will push the default new pipe with fluid volume
        // and push the default initial temperatures into the
        // temperature inlet and outlet vectors
        self.fluid_entity_vector.push(new_pipe);
        self.inlet_temp_vec.push(self.initial_global_temp.clone());
        self.outlet_temp_vec.push(self.initial_global_temp.clone());

        // i will then push the default work done and heat input into
        // these vectors

        self.work_input_vec.push(
            Power::new::<watt>(0.0));
        self.heat_input_vec.push(
            Power::new::<watt>(0.0));


        // the mass flowrate is also by default set to zero

        self.mass_flowrate_vec.push(
            MassRate::new::<kilogram_per_second>(0.0));

    }
}

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
/// This trait helps the developer run through the steps
/// of enthalpy calculation
///
///
pub trait v2_ExplicitCalculationSteps {


    /// First Step: calculate enthalpies and bulk fluid temp
    /// from temperatures
    fn step_1_calculate_current_timestep_temp_enthalpies(
        &mut self);

    /// Second Step: calculate new outlet enthalpy from available
    /// enthalpies, heat loss/gain and work done rates
    /// will probably require timestep also
    fn step_2_calculate_new_outlet_enthalpy(
        &mut self, 
        heat_supplied_to_fluid: Power,
        work_done_on_fluid: Power,
        timestep: Time,
        fluid_mass_flowrate: MassRate);

    /// third step: calculate new outlet
    /// temperature based on new outlet enthalpy
    ///
    /// we will then obtain the inlet and outlet
    /// temperatures based on the outlet temperatures
    /// of each pipe using some matrix solver
    /// which should give us a vector of inlet
    /// temperatures
    fn step_3_calculate_new_outlet_temperature(
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
    fn step_5_set_outlet_temperature(
        &mut self,
        new_outlet_temperature: ThermodynamicTemperature);


    /// now that we have all the required information,
    /// we can set the old temperatures to the values of
    /// the new temperatures
    fn step_6_update_current_timestep_temperatures(
        &mut self);
}


/// old version of explicit calculation steps,
/// i thought it was okay to calculate new system temperature
/// T_sys = (T_in+T_out)/2
///
/// it turns out that this method does not conserver energy
/// and so i will have to use the well mixed assumption
pub trait v1_ExplicitCalculationSteps {


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
    fn step_5_set_outlet_temperature(
        &mut self,
        new_outlet_temperature: ThermodynamicTemperature);


    /// now that we have all the required information,
    /// we can set the old temperatures to the values of
    /// the new temperatures
    fn step_6_update_current_timestep_temperatures(
        &mut self);
}

