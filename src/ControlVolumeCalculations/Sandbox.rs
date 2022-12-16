#[warn(missing_docs)]

#[cfg(test)]
mod sandbox_therminol_dowtherm_pipes {
    /// from this sandbox, i am experimenting with 
    /// using a struct and its methods to build my therminolPipes
    /// and run through their calculation steps at every timestep
    ///
    /// so far i've learnt that:
    ///
    /// (1) it's better to calculate the outlet temperature directly
    /// rather than go for fluid temperature; the using
    /// T_fluid = (T_in+T_out)/2 does not conserve energy
    ///
    /// (2) let the mutable inlet temperature and outlet temperautre
    /// vectors be a a part of the struct, same goes for the calculator
    ///
    /// (3) index numbers and vectors to keep track of inlet and outlet
    /// temperatures seems pretty okay!
    ///
    /// (4) setting inlet and outlet temperatures should be the same
    /// step
    ///
    /// (5) use usize instead of i32 to index vectors, though i32 
    /// can be readily converted into usize.
    ///
    /// (6) power, massfow and work done vectors can be initiated in the struct
    /// as well
    /// 
    /// (7) we can set a global timestep and global time in our 
    /// struct
    ///
    /// (8) we may want to have get and set methods for power/work done
    /// and mass flow
    ///
    /// for future consideration:
    ///
    /// (1) the inputs for the calculator function may need to extend
    /// beyond simple pipes if we are to include heat exchangers
    /// and other things
    ///
    /// (2) we might want to implement an ambient .cool() function
    /// in order to represent heat loss to environment
    #[test]
    pub fn sandbox_autobuild_conenct_and_autocalculation() {
        extern crate approx;

        use crate::ControlVolumeCalculations::TherminolDowthermPipes::
            FixedHeatFluxTherminolPipe;

        use crate:: ControlVolumeCalculations::FluidEntity_StructsAndTraits::
            FluidEntityInitialisationSteps;


        use uom::si::f64::*;
        use uom::si::time::second;
        use uom::si::thermodynamic_temperature::kelvin;
        use uom::si::volume::cubic_meter;


        let timestep = Time::new::<second>(0.1_f64);
        let initial_global_temp = ThermodynamicTemperature::
            new::<kelvin>(300_f64);
        let fluid_volume = Volume::new::<cubic_meter>(
            0.01_f64.powf(3_f64));

        // first thing first, i want to streamline the pipe creation process
        // (and of course, eventually place those hydrodynamic parameters as well,
        // but will probably think about that later)
        // 
        // and i also want to use a for loop to do these calculation steps
        // hence, putting these in a vector form will be good
        // so i will have 3 vectors,
        // one vector for inlet temperatures
        // one vector for outlet temperatures
        // one vector for all the FixedHeatFluxTherminolPipe

        let mut therminolPipeVec: Vec<FixedHeatFluxTherminolPipe> 
            = vec![];

        let mut inlet_temp_vec: Vec<ThermodynamicTemperature>
            = vec![];

        let mut outlet_temp_vec: Vec<ThermodynamicTemperature>
            = vec![];

        pub struct HeatFluxPipeFactory {
            pub current_max_index: i32,
        }

        impl HeatFluxPipeFactory {

            pub fn new() -> Self {

                return Self { current_max_index: 0 };

            }

            pub fn add_new_component(
                &mut self,
                timestep: Time,
                initial_global_temp: ThermodynamicTemperature,
                fluid_volume: Volume,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>,
                inlet_temp_vec: &mut Vec<ThermodynamicTemperature>,
                outlet_temp_vec: &mut Vec<ThermodynamicTemperature>
                ) {

                let mut new_pipe = FixedHeatFluxTherminolPipe::new();

                let fluid_entity_index = self.current_max_index;

                new_pipe.step_0_set_timestep_and_initial_temperatures(
                    timestep,
                    initial_global_temp,
                    fluid_volume,
                    fluid_entity_index);

                generic_component_vec.push(new_pipe);
                inlet_temp_vec.push(initial_global_temp);
                outlet_temp_vec.push(initial_global_temp);

                self.current_max_index = fluid_entity_index + 1;

            }

        }

        let mut factory_obj = HeatFluxPipeFactory::new();

        // so this is pipe 1

        factory_obj.add_new_component(
            timestep,
            initial_global_temp,
            fluid_volume,
            &mut therminolPipeVec,
            &mut inlet_temp_vec,
            &mut outlet_temp_vec);

        // only issue here, no name for each component, may be difficult
        // to differentiate


        // so this is pipe 2

        factory_obj.add_new_component(
            timestep,
            initial_global_temp,
            fluid_volume,
            &mut therminolPipeVec,
            &mut inlet_temp_vec,
            &mut outlet_temp_vec);


        // so this is pipe 3

        factory_obj.add_new_component(
            timestep,
            initial_global_temp,
            fluid_volume,
            &mut therminolPipeVec,
            &mut inlet_temp_vec,
            &mut outlet_temp_vec);

        // the next step is to connect each pipe
        // so i can try giving a factory method which helps to connect
        // two pipe objects
        // for this i will need a pipe vector, 

        // the index of the pipe which
        // we are interested to connect at the outlet
        // and the pipe we are interested to connect at the inlet

        impl HeatFluxPipeFactory {

            pub fn connect_inlet_and_outlet_pipe(
                &self,
                connect_to_pipe_outlet_index: usize,
                connect_to_pipe_inlet_index: usize,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>){


                // Basically in this function, i cannot use borrow
                // two mutable versions of the component vector and then
                // change values in them
                // i have to make a copy of the front and back pipe
                // and then use those to perform the value changes
                // the other way of course, is to change the value indices manually


                let mut pipe_back = 
                    generic_component_vec[connect_to_pipe_outlet_index].clone();

                let mut pipe_front = 
                    generic_component_vec[connect_to_pipe_inlet_index].clone();

                pipe_front.step_1_connect_to_component_inlet(
                    &mut generic_component_vec[connect_to_pipe_outlet_index]);

                pipe_back.step_2_conenct_to_component_outlet(
                    &mut generic_component_vec[connect_to_pipe_inlet_index]);

            }


        }

        // I can use the above method to connect my pipes!
        // i have it this way:
        // 1 -> 2 -> 3 
        // and 3 connects back to 1 in a circular fashion

        // however pipe 1 has an index of 0,
        // pipe 2 has an index of 1 and so on...

        factory_obj.connect_inlet_and_outlet_pipe(0,1,
                                                  &mut therminolPipeVec);

        factory_obj.connect_inlet_and_outlet_pipe(1,2,
                                                  &mut therminolPipeVec);

        factory_obj.connect_inlet_and_outlet_pipe(2,0,
                                                  &mut therminolPipeVec);


        // in doing so, i have connected all the pipes and these
        // pipes will know the correct index

        // all right, now let's take pipe1 for example,
        // its own index should be 0
        // the pipe connected to its back is index 2,
        // the pipe connected to its front is index 1
        // i'm going to repeat it all for the pipes involved

        let mut pipe1 = therminolPipeVec[0].clone();

        assert_eq!(0, pipe1.fluid_parameters.index_data.fluid_entity_index);

        assert_eq!(2, pipe1.fluid_parameters.index_data.
                   inlet_fluid_entity_index);

        assert_eq!(1, pipe1.fluid_parameters.index_data.
                   outlet_fluid_entity_index);

        let mut pipe2 = therminolPipeVec[1].clone();

        assert_eq!(1, pipe2.fluid_parameters.index_data.
                   fluid_entity_index);

        assert_eq!(0, pipe2.fluid_parameters.index_data.
                   inlet_fluid_entity_index);

        assert_eq!(2, pipe2.fluid_parameters.index_data.
                   outlet_fluid_entity_index);

        let mut pipe3 = therminolPipeVec[2].clone();

        assert_eq!(2, pipe3.fluid_parameters.index_data.
                   fluid_entity_index);
        assert_eq!(1, pipe3.fluid_parameters.index_data.
                   inlet_fluid_entity_index);
        assert_eq!(0, pipe3.fluid_parameters.index_data.
                   outlet_fluid_entity_index);


        // now that we have connected the pipes, we can start
        // calculating
        // we can perhaps create two vectors,
        // one for work done, one for heat input in fluid
        // one for work done on fluid
        //
        use crate::ControlVolumeCalculations::ExplictCalculations::
            ExplicitCalculationSteps;

        impl HeatFluxPipeFactory {

            pub fn step_1_calculate_current_timestep_temp_enthalpies(&self,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>){

                // start function

                let max_vec_index = 
                    generic_component_vec.len() - 1;

                for i in 0..=max_vec_index {
                    generic_component_vec[i].
                        step_1_calculate_current_timestep_temp_enthalpies();

                }


                // end of function
            }

            pub fn step_2_calculate_new_system_enthalpy(
                &self,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>,
                heat_input_vec: Vec<Power>,
                work_input_vec: Vec<Power>,
                mass_flowrate_vec: Vec<MassRate>,
                timestep: Time) {

                // start function
                
                let max_vec_index = 
                    generic_component_vec.len() - 1;

                for i in 0..=max_vec_index {

                    // first let's get the heat rate:
                    //
                    let heat_input_into_fluid : Power = 
                        heat_input_vec[i];

                    // now work done rate:

                    let work_input_into_fluid : Power = 
                        work_input_vec[i];

                    // now mass flowrate:
                    //

                    let mass_flowrate: MassRate = 
                        mass_flowrate_vec[i];




                    generic_component_vec[i].
                        step_2_calculate_new_system_enthalpy(
                            heat_input_into_fluid,
                            work_input_into_fluid,
                            timestep,
                            mass_flowrate);

                }

                // end of function
            }

            pub fn step_3_calculate_new_system_temperature(
                &mut self,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>){


                let max_vec_index = 
                    generic_component_vec.len() - 1;

                for i in 0..=max_vec_index {
                    generic_component_vec[i].
                        step_3_calculate_new_system_temperature();

                }



            }

        }

        // we can then use this function to help us with step 1
        //

        factory_obj.step_1_calculate_current_timestep_temp_enthalpies(
            &mut therminolPipeVec);

        // now for calculation
        let timestep : Time = pipe1.fluid_parameters.timestep;

        use uom::si::mass_rate::kilogram_per_second;

        let mass_flowrate: MassRate = 
            MassRate::new::<kilogram_per_second>(0.18);

        use uom::si::power::watt;
        
        let pipe_1_heat = Power::new::<watt>(100_f64);
        let pipe_2_heat = Power::new::<watt>(-20_f64);
        let pipe_3_heat = Power::new::<watt>(-80_f64);
        
        let work_done_on_pipe_rate = Power::new::<watt>(0_f64);

        // let's construct the vectors
        //

        let mut heat_input_vec : Vec<Power> =
            vec![];

        let mut work_input_vec : Vec<Power> =
            vec![];

        let mut mass_flowrate_vec: Vec<MassRate> = 
            vec![];

        // push values into vector

        heat_input_vec.push(pipe_1_heat);
        heat_input_vec.push(pipe_2_heat);
        heat_input_vec.push(pipe_3_heat);

        work_input_vec.push(work_done_on_pipe_rate);
        work_input_vec.push(work_done_on_pipe_rate);
        work_input_vec.push(work_done_on_pipe_rate);

        mass_flowrate_vec.push(mass_flowrate);
        mass_flowrate_vec.push(mass_flowrate);
        mass_flowrate_vec.push(mass_flowrate);


        // next thing, complete step 2

        factory_obj.step_2_calculate_new_system_enthalpy(
            &mut therminolPipeVec,
            heat_input_vec,
            work_input_vec,
            mass_flowrate_vec,
            timestep);

        // step 3

        factory_obj.step_3_calculate_new_system_temperature(
            &mut therminolPipeVec);


        // now at this point, there is still some correction to 
        // make with the FixedHeatFluxTherminolPipe
        //
        // since the enthalpy balances do not corroborate well
        // 
        // but the new fluid temperatures should now reflect
        // the outlet temperatures of the pipes
        // Hence, i will take the system temperatures of the 
        // pipe and assign it appropriately to the vector of
        // the outlet temperatures

        impl HeatFluxPipeFactory {

            pub fn step_4_set_inlet_temperature(
                &mut self,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>,
                inlet_temp_vec: &mut Vec<ThermodynamicTemperature>,
                outlet_temp_vec: &mut Vec<ThermodynamicTemperature>){

                // first let's clear up the inlet and outlet temp vector

                inlet_temp_vec.clear();
                outlet_temp_vec.clear();

                // now let's update all the outlet temperatures

                let max_vec_index = 
                    generic_component_vec.len() - 1;

                for i in 0..=max_vec_index {
                    
                    // first let's obtain the outlet temperature
                    //
                    let fluid_component =
                        generic_component_vec[i].clone();

                    let fluid_component_outlet_temperature =
                        fluid_component.fluid_parameters.
                        temperature_data.fluid_temp_new;

                    // we'll introduce it into the vector

                    outlet_temp_vec.push(fluid_component_outlet_temperature);

                    // of course, we can set the outlet temperature here outright,
                    // but i'll leave it for later

                }

                // now that we've set the outlet temperature
                // vectors, we can use the indexing within each object
                // to assign them to the inlet temperatures

                for i in 0..=max_vec_index {

                    // first let's obtain the index
                    // of the component connected to this component's
                    // inlet
                    
                    let fluid_component =
                        generic_component_vec[i].clone();

                    let fluid_component_inlet_index : usize =
                        fluid_component.fluid_parameters.
                        index_data.inlet_fluid_entity_index
                        .try_into().unwrap();

                    // second, we get the outlet temperature of the fluid
                    // component connected to the back of this fluid
                    // component
                    //
                    // This is actually the inlet temperature

                    let fluid_component_inlet_temperature =
                        outlet_temp_vec[fluid_component_inlet_index];

                    // third, let's push this temperature to
                    // the inlet temperature vector

                    inlet_temp_vec.push(fluid_component_inlet_temperature);

                }

                // now we have set both inlet and outlet temperature vectors,
                // we can start assigning inlet temperatures

                for i in 0..=max_vec_index {

                    generic_component_vec[i].fluid_parameters.
                        temperature_data.inlet_temp_new 
                        = inlet_temp_vec[i].clone();

                }

                return;


            }

            pub fn step_5_set_outlet_temperature(
                &mut self,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>,
                outlet_temp_vec: &mut Vec<ThermodynamicTemperature>){

                let max_vec_index = 
                    generic_component_vec.len() - 1;

                for i in 0..=max_vec_index {

                    generic_component_vec[i].fluid_parameters.
                        temperature_data.outlet_temp_new =
                        outlet_temp_vec[i].clone();

                }

                return;
            }

            // end of impl
        }


        factory_obj.step_4_set_inlet_temperature(
            &mut therminolPipeVec,
            &mut inlet_temp_vec,
            &mut outlet_temp_vec);

        factory_obj.step_5_set_outlet_temperature(
            &mut therminolPipeVec,
            &mut outlet_temp_vec);

        // now let's do some asserts

        let mut pipe1 = therminolPipeVec[0].clone();
        let mut pipe2 = therminolPipeVec[1].clone();
        let mut pipe3 = therminolPipeVec[2].clone();

        // for reference, the outlet temperatures are as follows
        // T_new1 = 305.91 K
        // T_new2 = 298.8 K
        // T_new3 = 295.2 K


        let temp_1_val = 305.91_f64;
        let temp_2_val = 298.8_f64;
        let temp_3_val = 295.2_f64;

        approx::assert_relative_eq!(
            temp_1_val,
            pipe1.fluid_parameters.temperature_data.outlet_temp_new.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_2_val,
            pipe2.fluid_parameters.temperature_data.outlet_temp_new.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_3_val,
            pipe3.fluid_parameters.temperature_data.outlet_temp_new.value,
            max_relative=0.01);

        // likewise, let's assert the new inlet temperatures too

        approx::assert_relative_eq!(
            temp_1_val,
            pipe2.fluid_parameters.temperature_data.inlet_temp_new.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_2_val,
            pipe3.fluid_parameters.temperature_data.inlet_temp_new.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_3_val,
            pipe1.fluid_parameters.temperature_data.inlet_temp_new.value,
            max_relative=0.01);

        impl HeatFluxPipeFactory {
            
            pub fn step_6_update_current_timestep_temperatures(
                &mut self,
                generic_component_vec: &mut Vec<FixedHeatFluxTherminolPipe>){


                let max_vec_index = 
                    generic_component_vec.len() - 1;

                for i in 0..=max_vec_index {
                    generic_component_vec[i].
                        step_6_update_current_timestep_temperatures();

                }

                return;
            }

            // end of impl
        }

        // now we change the old temperature data (from last timestep)
        // to the new temperature data (current timestep)
        factory_obj.step_6_update_current_timestep_temperatures(
            &mut therminolPipeVec);

        pipe1 = therminolPipeVec[0].clone();
        pipe2 = therminolPipeVec[1].clone();
        pipe3 = therminolPipeVec[2].clone();

        approx::assert_relative_eq!(
            temp_1_val,
            pipe1.fluid_parameters.temperature_data.outlet_temp_old.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_2_val,
            pipe2.fluid_parameters.temperature_data.outlet_temp_old.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_3_val,
            pipe3.fluid_parameters.temperature_data.outlet_temp_old.value,
            max_relative=0.01);

        // likewise, let's assert the old inlet temperatures too

        approx::assert_relative_eq!(
            temp_1_val,
            pipe2.fluid_parameters.temperature_data.inlet_temp_old.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_2_val,
            pipe3.fluid_parameters.temperature_data.inlet_temp_old.value,
            max_relative=0.01);

        approx::assert_relative_eq!(
            temp_3_val,
            pipe1.fluid_parameters.temperature_data.inlet_temp_old.value,
            max_relative=0.01);
    }

}


