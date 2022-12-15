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
/// This is the primitive version of calculating 
/// enthalpies and temperatures..
///
///
///
/// PART 2:
///
/// This trial and error bit helps to streamline the temperature assignment
/// ```rust
/// let a = 1;
/// ```
///
/// PART 1:
/// This is the trial and error bit where i am
/// sort of explaining my thought process for code development
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
/// let mut fluid_entity_index: i32 = 1;
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
/// // now suppose there are 3 pipes and i want to connect them
///
/// let mut pipe2 = FixedHeatFluxTherminolPipe::new();
///
/// pipe2.step_0_set_timestep_and_initial_temperatures(
/// timestep,
/// initial_global_temp,
/// fluid_volume,
/// 2);
///
/// let mut pipe3 = FixedHeatFluxTherminolPipe::new();
///
/// pipe3.step_0_set_timestep_and_initial_temperatures(
/// timestep,
/// initial_global_temp,
/// fluid_volume,
/// 3);
///
///
/// // This section tests if the pipe connection indexing is working 
/// // correctly
/// 
/// // let pipe 2 be connected to the outlet of pipe 1
///
/// pipe1.step_2_conenct_to_component_outlet(&mut pipe2);
///
///
/// // we shall also connect pipe3 to the outlet of pipe2
///
/// pipe2.step_2_conenct_to_component_outlet(&mut pipe3);
///
/// // lastly we shall connect pipe3 to the inlet of pipe1
///
/// pipe1.step_1_connect_to_component_inlet(&mut pipe3);
///
/// // so pipe 1's index should be 1
///
/// assert_eq!(1, pipe1.fluid_parameters.index_data.fluid_entity_index);
///
///
/// // hence the index of the pipe connected to pipe 1's inlet should
/// // be the index of pipe 3 (which is 3)
///
/// assert_eq!(3, pipe1.fluid_parameters.index_data.inlet_fluid_entity_index);
///
/// // finally, the outlet index of pipe 1 should be 2
///
/// assert_eq!(2, pipe1.fluid_parameters.index_data.outlet_fluid_entity_index);
///
///
/// // and for pipe 3, it's index should be 3,
/// // its inlet pipe index should be 2,
/// // and its outlet pipe index should be 1
///
/// assert_eq!(3, pipe3.fluid_parameters.index_data.fluid_entity_index);
/// assert_eq!(2, pipe3.fluid_parameters.index_data.inlet_fluid_entity_index);
/// assert_eq!(1, pipe3.fluid_parameters.index_data.outlet_fluid_entity_index);
///
/// // for pipe 2, the index is 2, pipe 1 is connected to its inlet
/// // and pipe 3 to its outlet
///
/// assert_eq!(2, pipe2.fluid_parameters.index_data.fluid_entity_index);
/// assert_eq!(1, pipe2.fluid_parameters.index_data.inlet_fluid_entity_index);
/// assert_eq!(3, pipe2.fluid_parameters.index_data.outlet_fluid_entity_index);
///
/// // The last part of the tests stipulates that we should 
/// // populate a vector. This vector should be used in a for loop
/// // so that we can autocalculate the temperatures at each time step
/// // 
/// // but for us to know how to calculate things, we'll probably want
/// // to do it manually first, to best know how to operate
/// // I am supposing that pipe 1 has a supply of 100 W
/// // pipe 2 has a heat loss of 20 W and pipe 3 has a heat loss of 80 W
/// // cannot do too much power because it will cause the temperature
/// // to go below 20C, which is out of range
///
/// use uom::si::power::watt;
///
/// let pipe_1_heat = Power::new::<watt>(100_f64);
/// let pipe_2_heat = Power::new::<watt>(-20_f64);
/// let pipe_3_heat = Power::new::<watt>(-80_f64);
///
/// let work_done_on_pipe_rate = Power::new::<watt>(0_f64);
/// 
/// // let's begin step 1 of calculation procedure
///
/// use crate::heat_transfer_rust::ControlVolumeCalculations::
/// ExplictCalculations::ExplicitCalculationSteps;
///
///
/// pipe1.step_1_calculate_current_timestep_temp_enthalpies();
/// pipe2.step_1_calculate_current_timestep_temp_enthalpies();
/// pipe3.step_1_calculate_current_timestep_temp_enthalpies();
///
/// // then step 2, which is to calculate new system enthalpy
/// // we assume mass flowrate has already been calculated 
/// // for this timestep or is constant
/// 
/// use uom::si::mass_rate::kilogram_per_second;
///
/// let timestep : Time = pipe1.fluid_parameters.timestep;
/// let mass_flowrate : MassRate = MassRate::new::
/// <kilogram_per_second>(0.18);
///
/// pipe1.step_2_calculate_new_system_enthalpy(
/// pipe_1_heat,
/// work_done_on_pipe_rate,
/// timestep,
/// mass_flowrate);
///
///
/// pipe2.step_2_calculate_new_system_enthalpy(
/// pipe_2_heat,
/// work_done_on_pipe_rate,
/// timestep,
/// mass_flowrate);
///
/// 
/// pipe3.step_2_calculate_new_system_enthalpy(
/// pipe_3_heat,
/// work_done_on_pipe_rate,
/// timestep,
/// mass_flowrate);
///
///
///
/// // now for step 3, to calculate new thermodynamic temperature
/// // for now i'll have to debug the therminol properties, it's not
/// // giving the correct number
///
/// let T_new1 = pipe1.step_3_calculate_new_system_temperature();
/// let T_new2 = pipe2.step_3_calculate_new_system_temperature();
/// let T_new3 = pipe3.step_3_calculate_new_system_temperature();
///
/// let mut error_string: String = T_new1.value.to_string();
///
/// error_string.push_str(" ");
/// error_string.push_str(&T_new2.value.to_string());
/// error_string.push_str(" ");
/// error_string.push_str(&T_new3.value.to_string());
///
/// // panic!("{}", error_string);
///
/// // at time of test
/// // T_new1 = 305.91 K
/// // T_new2 = 298.8 K
/// // T_new3 = 295.2 K
///
/// approx::assert_relative_eq!(305.9, T_new1.value, max_relative=0.001);
/// approx::assert_relative_eq!(298.8, T_new2.value, max_relative=0.001);
/// approx::assert_relative_eq!(295.2, T_new3.value, max_relative=0.001);
///
/// //panic!("{}",pipe2.fluid_parameters.temperature_data.
/// //fluid_temp_new.value);
///
/// // to calculate for step 4, i will need to solve a few eqns
/// //  T_new1 (kelvin) = (T_in1 + T_out1)/2
/// //  T_new2 (kelvin) = (T_in2 + T_out2)/2
/// //  T_new3 (kelvin) = (T_in3 + T_out3)/2
/// // 
/// // However, T_in1 = T_out3
/// // T_in2 = T_out_1
/// // T_in3 = T_out2
///
///
/// //  In terms of inlet temperatures, this becomes:
/// //  T_new1 (kelvin) = (T_in1 + T_in2)/2
/// //  T_new2 (kelvin) = (T_in2 + T_in3)/2
/// //  T_new3 (kelvin) = (T_in3 + T_in1)/2
///
/// // if i were to represent this in matrix form
/// //  |0.5 0.5 0.0| | T_in1 |   | T_new1 |
/// //  |0.0 0.5 0.5| | T_in2 | = | T_new2 |
/// //  |0.5 0.0 0.5| | T_in3 |   | T_new3 |
/// //
/// // we then solve this matrix
/// // There are two problems here:
/// // (1) How do we construct this matrix automatically,
/// // (2) How do we solve this matrix automatically
/// // 
///
/// // For now let's solve the second problem first
/// // for reference
/// // solving this results in (i used scilab linsolve)
/// // T_in1 = 302   K
/// // T_in2 = 309.5 K
/// // T_in3 = 288.1 K
/// // 
/// // the way forward:
///
/// extern crate ndarray;
/// use ndarray::prelude::*;
/// use ndarray_linalg::Solve;
///
/// // first we make a 2D array 
/// let matrixA : Array2<f64> = 
/// array![
/// [0.5, 0.5, 0.0],
/// [0.0, 0.5, 0.5],
/// [0.5, 0.0, 0.5]
/// ];
///
/// let vectorB: Array1<f64> = 
/// array![ 305.91_f64, 298.8, 295.2 ];
/// //
/// // note that for this to work, i needed to install
/// // ndarray_linalg with features
/// // openblas-static
/// // https://github.com/rust-ndarray/ndarray-linalg/issues/171
/// // This means i needed gcc-fortran on my archlinux as well
///
/// let x = matrixA.solve_into(vectorB).unwrap();
///
/// approx::assert_relative_eq!(302_f64, x[0], max_relative=0.01);
/// approx::assert_relative_eq!(309.5_f64, x[1], max_relative=0.01);
/// approx::assert_relative_eq!(288.1_f64, x[2], max_relative=0.01);
///
///
/// // now this solves the thing ok, and it means that there is a 7.5 C
/// // increase in pipe 1,
/// // a 21.4C decrease in pipe 2
/// // and a 13.9 C increase in pipe 3
/// // this makes no physical sense
///
/// // as compared to pipe 1 outlet being 305.91K, inlet is 295.2K
/// // pipe 2 outlet 298.8K, inlet 305.91K,
/// // pipe 3 outlet 295.2K, inlet 298.8K 
/// // in the former case, energy is not conserved
/// // therefore, we shall not use the fluid temp
/// // T_fluid = (T_in + T_out)/2.0 
/// // for use in energy balance
/// // thus, i will use the outlet temperature to be the same
/// // temperature as the control volume temperature
/// // similar to a CSTR (continuously stirred tank reactor)
/// // and we assume perfect mixing
/// 
/// // step 4 and 5
/// // for the inlet temperatures we can use the T_new1, T_new2, and T_new3
/// // as outlet temps for pipe 1,2 and 3
///
/// pipe1.step_4_set_inlet_temperature(T_new3);
/// pipe2.step_4_set_inlet_temperature(T_new1);
/// pipe3.step_4_set_inlet_temperature(T_new2);
///
/// pipe1.step_5_set_outlet_temperature(T_new1);
/// pipe2.step_5_set_outlet_temperature(T_new2);
/// pipe3.step_5_set_outlet_temperature(T_new3);
///
/// // step 6: now we set the old inlet and outlet temperatures
///
/// pipe1.step_6_update_current_timestep_temperatures();
/// pipe2.step_6_update_current_timestep_temperatures();
/// pipe3.step_6_update_current_timestep_temperatures();
///
/// // if everything works out well, the old temperatures
/// // should reflect the new values at the next time step
///
///
/// approx::assert_relative_eq!(305.9, 
/// pipe1.fluid_parameters.temperature_data.outlet_temp_old.value, 
/// max_relative=0.001);
///
/// approx::assert_relative_eq!(298.8, 
/// pipe2.fluid_parameters.temperature_data.outlet_temp_old.value, 
/// max_relative=0.001);
///
/// approx::assert_relative_eq!(295.2, 
/// pipe3.fluid_parameters.temperature_data.outlet_temp_old.value, 
/// max_relative=0.001);
///
/// // now we test the inlet temperatures
///
/// approx::assert_relative_eq!(305.9, 
/// pipe2.fluid_parameters.temperature_data.inlet_temp_old.value, 
/// max_relative=0.001);
///
/// approx::assert_relative_eq!(298.8, 
/// pipe3.fluid_parameters.temperature_data.inlet_temp_old.value, 
/// max_relative=0.001);
///
/// approx::assert_relative_eq!(295.2, 
/// pipe1.fluid_parameters.temperature_data.inlet_temp_old.value, 
/// max_relative=0.001);
/// 
/// // this concludes the testing for one round of a 3 pipe problem
/// // we might want to do the temperature assignment in a more streamlined
/// // manner
///
/// ```
///
///
///
///
///
pub struct FixedHeatFluxTherminolPipe {
    pub fluid_parameters: FluidEntityThermophysicalData,
}

impl FixedHeatFluxTherminolPipe {

    /// Constructor which creates the structure 
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
        other_fluid_entity: &mut Self) {

        FluidEntityThermophysicalData::
            step_1_connect_to_component_inlet(
                &mut self.fluid_parameters, 
                &mut other_fluid_entity.fluid_parameters);
    }

    fn step_2_conenct_to_component_outlet(
         &mut self,
         other_fluid_entity: &mut Self){

        FluidEntityThermophysicalData::
            step_2_conenct_to_component_outlet(
                &mut self.fluid_parameters, 
                &mut other_fluid_entity.fluid_parameters);

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
    ///
    /// the fluid properties except for enthalpy
    /// are calculated at T_fluid = (T_in - T_out)/2
    ///
    ///
    /// fluid enthalpy wise, we assume perfect mixing
    /// which means the bulk temperature of the pipe is
    /// the same as the exit temperature
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
            outlet_enthalpy_old;

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

        // set inlet and outlet temperatures
        self.fluid_parameters.temperature_data.
            inlet_temp_old =
            self.fluid_parameters.temperature_data.
            inlet_temp_new.clone();

        self.fluid_parameters.temperature_data.
            fluid_temp_old =
            self.fluid_parameters.temperature_data.
            fluid_temp_new.clone();


        // set new fluid temperature to average
        // of new inlet and outlet temperatures

        let new_fluid_temp_value =
            (self.fluid_parameters.temperature_data.
            inlet_temp_new.value + 
            self.fluid_parameters.temperature_data.
            outlet_temp_new.value)/2.0;

        self.fluid_parameters.temperature_data.
            fluid_temp_new = 
            ThermodynamicTemperature::new::
            <kelvin>(new_fluid_temp_value);


        self.fluid_parameters.temperature_data.
            outlet_temp_old =
            self.fluid_parameters.temperature_data.
            outlet_temp_new.clone();

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

