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

    pub component_name_vec: Vec<String>,

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
            component_name_vec: vec![],

            /// Default timestep is 0.1s
            timestep: Time::new::<second>(0.1),
            /// Default initial global temp is 300k or about 27C
            initial_global_temp: ThermodynamicTemperature::new::
                <kelvin>(300.0),
        }

    }

    pub fn setup_step_0_set_timestep_and_initial_temp(
        &mut self,
        timestep: Time,
        initial_global_temp: ThermodynamicTemperature) {

        self.timestep = timestep;
        self.initial_global_temp = initial_global_temp;
    }



    /// Adds a new fluid component or fluid entity
    pub fn setup_step_1_add_new_component(
        &mut self,
        name: String,
        fluid_volume: Volume
        ) {

        // first let me push the name of the fluid entity up
        self.component_name_vec.push(name);

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

        return;

    }

    /// setup step : connect inlet and outlet of pipe
    pub fn setup_step_2_connect_inlet_and_outlet_pipe(
        &mut self,
        connect_to_pipe_outlet_index: usize,
        connect_to_pipe_inlet_index: usize){


        // Basically in this function, i cannot use borrow
        // two mutable versions of the component vector and then
        // change values in them
        // i have to make a copy of the front and back pipe
        // and then use those to perform the value changes
        // the other way of course, is to change the value indices manually


        let mut pipe_back = 
            self.fluid_entity_vector[connect_to_pipe_outlet_index].clone();

        let mut pipe_front = 
            self.fluid_entity_vector[connect_to_pipe_inlet_index].clone();

        pipe_front.step_1_connect_to_component_inlet(
            &mut self.fluid_entity_vector[connect_to_pipe_outlet_index]);

        pipe_back.step_2_conenct_to_component_outlet(
            &mut self.fluid_entity_vector[connect_to_pipe_inlet_index]);

    }

    /// Step 2: set mass flowrate for a component with 
    /// index i
    pub fn step_2_set_mass_flowrate(
        &mut self,
        mass_flowrate: MassRate,
        component_index: usize){

        self.mass_flowrate_vec[component_index] = mass_flowrate.clone();
    }

    /// Step 3: set work input vector for component with 
    /// index i
    pub fn step_3_set_work_input(
        &mut self,
        work_input: Power,
        component_index: usize){

        self.work_input_vec[component_index] = work_input.clone();
    }

    /// Step 4: set heat input vector for component with 
    /// index i
    pub fn step_4_set_heat_input(
        &mut self,
        heat_input: Power,
        component_index: usize){

        self.heat_input_vec[component_index] = heat_input.clone();
    }

    /// Step 5: calculate outlet enthalpy
    /// in a serial manner (not worrying about parallel
    /// computation with rayon yet)
    pub fn step_5_calculate_all_outlet_enthalpies_and_temperatures(
        &mut self) {

        // start the for loop
        let max_vec_index_plus_one = 
            self.fluid_entity_vector.len();

        for i in 0..max_vec_index_plus_one {

            let heat_input_into_fluid : Power = 
                self.heat_input_vec[i];

            let work_done_on_fluid : Power = 
                self.work_input_vec[i];

            let mass_flowrate: MassRate = 
                self.mass_flowrate_vec[i];

            self.fluid_entity_vector[i].
                step_2_calculate_new_outlet_enthalpy(
                    heat_input_into_fluid,
                    work_done_on_fluid,
                    self.timestep,
                    mass_flowrate);

            self.fluid_entity_vector[i].
                step_3_calculate_new_outlet_temperature();
        }
        return;

    }

    /// Step 6: calculate inlet temperatures and assign them
    /// to appropriate vectors
    pub fn step_6_calculate_inlet_temperatures(
        &mut self){

        // first let's clear up the inlet and outlet temp vector

        self.inlet_temp_vec.clear();
        self.outlet_temp_vec.clear();
        // now let's update all the outlet temperatures

        let max_vec_index_plus_one = 
            self.fluid_entity_vector.len();

        // first we update the outlet temp vector

        for i in 0..max_vec_index_plus_one {

            // first let's obtain the outlet temperature
            //
            let fluid_component =
                self.fluid_entity_vector[i].clone();

            let fluid_component_outlet_temperature =
                fluid_component.fluid_parameters.
                temperature_data.outlet_temp_new;

            // we'll introduce it into the vector

            self.outlet_temp_vec.push(fluid_component_outlet_temperature);

            // of course, we can set the outlet temperature here outright,
            // but i'll leave it for later


        }


        // now we update the inlet temp vector
        //
        for i in 0.. max_vec_index_plus_one {

            let fluid_component =
                self.fluid_entity_vector[i].clone();

            let fluid_component_inlet_index: usize = 
                fluid_component.fluid_parameters.
                index_data.inlet_fluid_entity_index;

            // second, we get the outlet temperature of the fluid
            // component connected to the back of this fluid
            // component
            //
            // This is actually the inlet temperature

            let fluid_component_inlet_temperature =
                self.outlet_temp_vec[fluid_component_inlet_index];

            // third, let's push this temperature to
            // the inlet temperature vector

            self.inlet_temp_vec.push(fluid_component_inlet_temperature);

        }


        // now we update the inlet and outlet temperatures
        // of each object
        //

        for i in 0..max_vec_index_plus_one {

            self.fluid_entity_vector[i].fluid_parameters.
                temperature_data.inlet_temp_new 
                = self.inlet_temp_vec[i].clone();

            self.fluid_entity_vector[i].fluid_parameters.
                temperature_data.outlet_temp_new =
                self.outlet_temp_vec[i].clone();
        }

        // and we are done!
        return;

    }


    /// Step 7: Advance timestep
    /// means i set the old temperature values to that of the new
    /// temperature
    pub fn step_7_advance_timestep(
        &mut self){

        let max_vec_index_plus_one = 
            self.fluid_entity_vector.len();

        for i in 0..max_vec_index_plus_one {
            self.fluid_entity_vector[i].
                step_6_update_current_timestep_temperatures();
        }
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

