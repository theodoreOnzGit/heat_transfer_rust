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

