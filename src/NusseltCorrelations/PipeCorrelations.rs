#![allow(non_snake_case)]

/// A nusselt correlation for CIET heater v1.0
///
/// it returns Nu = 8.0 
/// for Re < 2000.0
///
/// and returns Nu = 5.44 + 0.034*Re^(0.82)
/// for Re >= 2000.0
/// ```rust
/// extern crate approx;
/// use heat_transfer_rust::NusseltCorrelations::PipeCorrelations;
/// 
///
/// // for Re < 2000, return 8
/// let Re_laminar = 1500.0;
///
/// let Nu_laminar_test = PipeCorrelations::nusselt_ciet_heater_v1_0(Re_laminar);
///
/// approx::assert_relative_eq!(8.0, Nu_laminar_test, max_relative=0.001);
///
/// // the following two tests are taken from table 3-1 of:
/// // http://fhr.nuc.berkeley.edu/wp-content/uploads/2015/04/14-009_CIET-IRP-Final-Report.pdf
/// // this is page 33 out of 103 for the document
///
/// // this test is accurate to within 1% of stated value
///
/// let Re_turbulent = 2768_f64;
/// let Nu_turbulent_test = PipeCorrelations::
/// nusselt_ciet_heater_v1_0(Re_turbulent);
///
/// approx::assert_relative_eq!(28.0, Nu_turbulent_test, max_relative=0.01);
///
/// // this test is accurate to within 3% of stated value
///
/// let Re_turbulent_2 = 3932_f64;
/// let Nu_turbulent_test_2 = PipeCorrelations::
/// nusselt_ciet_heater_v1_0(Re_turbulent_2);
///
/// approx::assert_relative_eq!(36.0, Nu_turbulent_test_2, max_relative=0.03);
/// 
///
///
/// ```
///
/// Note that there is a discontinuity at Re = 2000
/// and that this is test bay data...
/// When heater was installed in CIET, there were different results
///
pub fn nusselt_ciet_heater_v1_0(Re: f64)-> f64 {

    if Re >= 2000_f64 {
        return 5.44 + 0.034*Re.powf(0.82);
    }

    return 8.0;

}


/// Dittus Boelter Correlation
///
/// https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf
///
///
/// Meant for turbulent flow
/// Smooth surface tubes
/// Heiss, J. F., & Coull, J. (1951). Nomograph of Dittus-Boelter 
/// equation for heating and cooling 
/// liquids. Industrial & Engineering Chemistry, 43(5), 1226-1229.
///
///
/// http://herve.lemonnier.sci.free.fr/TPF/NE/Winterton.pdf
///
/// The original paper is here
///
/// Dittus, F. W., & Boelter, L. M. K. (1985). Heat transfer in 
/// automobile radiators of the tubular type. International 
/// communications in heat and mass transfer, 12(1), 3-22.
///
/// The Dittus Boelter correlation has two forms,
/// one for heating and one for cooling
///
/// By heating I mean that the fluid is heated
/// and heat is transfered from the tube walls to the 
/// heater
///
/// And by cooling I mean that the fluid is cooled
/// and the wall takes heat from the fluid
///
/// ```rust
/// extern crate approx;
/// use heat_transfer_rust::NusseltCorrelations::PipeCorrelations;
///
/// // here we have an example for heating
/// // Re = 10000, Pr = 17
///
///
/// let Re = 10000_f64;
/// let Pr = 17_f64;
///
/// let heating_ref_nu = 0.023 * Re.powf(0.8) * Pr.powf(0.4);
///
/// let heating_test_bool = true;
///
/// let mut test_Nu = PipeCorrelations::dittus_boelter_correlation(Re, Pr,
/// heating_test_bool);
///
/// approx::assert_relative_eq!(heating_ref_nu, test_Nu, 
/// max_relative=0.01);
///
/// // here we have an example for cooling
/// // Re = 10000, Pr = 17
///
/// let cooling_ref_nu = 0.023 * Re.powf(0.8) * Pr.powf(0.3);
///
/// let cooling_test_bool = false;
///
/// test_Nu = PipeCorrelations::dittus_boelter_correlation(Re, Pr,
/// cooling_test_bool);
///
/// approx::assert_relative_eq!(cooling_ref_nu, test_Nu, 
/// max_relative=0.01);
/// ```
///
///https://www.nuclear-power.com/nuclear-engineering/heat-transfer/convection-convective-heat-transfer/sieder-tate-equation/
///
/// Unfortunately, Dittus Boelter correlation is valid
/// only for small to moderate temperature differences
///
/// For larger temperature differences, use Sieder-Tate
/// 
/// 
///
pub fn dittus_boelter_correlation(Re: f64, Pr: f64,
                                  heating: bool) -> f64 {

    if heating == true {
        let Nu = 0.023 * Re.powf(0.8) * Pr.powf(0.4);
        return Nu;
    }
    else {
        let Nu = 0.023 * Re.powf(0.8) * Pr.powf(0.3);
        return Nu;
    }

}

/// Sieder Tate Relationship
///
/// https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf
///
/// https://www.nuclear-power.com/nuclear-engineering/heat-transfer/convection-convective-heat-transfer/sieder-tate-equation/
///
/// Note that properties here are evaluated at Tavg (ie average bulk fluid
/// temperature)
///
/// For pipe or heat exchanger,
/// it could be 
///
/// Tavg = (T_outlet + T_inlet)/2
///
/// the Re, Pr is generally evaluated at fluid temperature
/// whereas the fluid viscosity ratio is the ratio of viscosity at
/// the bulk fluid temperature to 
/// fluid viscosity at wall temperature
///
/// Yang, X., Yang, X., Ding, J., Shao, Y., & Fan, H. (2012). 
/// Numerical simulation study on the heat transfer 
/// characteristics of the tube receiver of the 
/// solar thermal power tower. Applied Energy, 90(1), 142-147.
///
/// viscosity_ratio = mu_f / mu_s
///
/// note that this ratio is a dynamic viscosity ratio, not 
/// kinematic viscosity ratio
///
///
/// The range of applicability (from Perry's Handbook)
/// is 
/// 0.7 < Pr < 16700
/// and 
/// 4000 < Re_D <10000
///
/// and 
///
/// 0.0044 < viscosity_ratio <  9.75
///
/// The viscosity ratio bounds are estimated from the 
/// the seider tate laminar heat transfer correlation,
/// i assumed they are of the same bounds. Did not check
/// however.
/// 
/// This is for fully developed turbulent flow only
///
/// viscosity_ratio = 5.0;
///
///
/// ```rust
///
/// extern crate approx;
/// use heat_transfer_rust::NusseltCorrelations::PipeCorrelations;
///
/// let Re = 8000_f64;
/// let Pr = 17_f64;
///
/// // the viscosity ratio is assumed to be 5
///
/// let viscosity_ratio = 5.0_f64;
///
/// let nu_f_reference = 0.027 * Re.powf(0.8) 
/// * Pr.powf(1.0/3.0) * 
/// viscosity_ratio.powf(0.14);
///
/// let test_nu = PipeCorrelations::sieder_tate_correlation(
/// Re, Pr, viscosity_ratio);
///
/// approx::assert_relative_eq!(nu_f_reference, test_nu, 
/// max_relative=0.01);
///
/// ```
///
///
///
/// meant for turbulent flow
pub fn sieder_tate_correlation(Re: f64, Pr: f64, 
                               viscosity_ratio_fluid_over_wall: f64) -> f64 {

    if Pr < 0.7 {
        panic!("Sieder Tate Pr < 0.7, too low");
    }

    if Pr > 16700_f64 {
        panic!("Sieder Tate Pr > 16700, too high");
    }

    if Re < 4000_f64 {
        panic!("Sieder Tate Re < 4000, laminar or transition");
    }

    if Re > 10000_f64 {
        panic!("Sieder Tate Re > 10000, too high");
    }

    if viscosity_ratio_fluid_over_wall < 0.0044 {
        panic!("Sieder Tate viscosity_ratio_fluid_over_wall < 4000, 
               laminar or transition");
    }

    if viscosity_ratio_fluid_over_wall > 9.75 {
        panic!("Sieder Tate viscosity_ratio_fluid_over_wall > 
               10000, too high");
    }

    let Nu_f = 0.027 * Re.powf(0.8) * Pr.powf(0.33333333333) * 
        viscosity_ratio_fluid_over_wall.powf(0.14);

    return Nu_f;
}

/// Gnielinski Equation for liquids
///
///
/// https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf
///
/// turbulent flow, all kinds of tubes
///
/// However, flow should be fully developed
///
/// ```rust
///
/// extern crate approx;
/// use heat_transfer_rust::NusseltCorrelations::PipeCorrelations;
///
/// let Re = 8000_f64;
/// let Pr_fluid = 17_f64;
/// let Pr_wall = 12_f64;
/// let darcy_friction_factor = 0.005_f64;
///
/// // let's now calculate the nusslet number
///
/// let prandtl_ratio = Pr_fluid/Pr_wall;
///
/// let darcy_ratio: f64 = darcy_friction_factor/8.0;
///
/// let numerator: f64 = darcy_ratio * (Re - 1000_f64) * Pr_fluid *
///     prandtl_ratio.powf(0.11);
/// let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
///     (Pr_fluid.powf(2.0/3.0) - 1.0);
/// 
///
///
/// let nu_f_reference = numerator/denominator;
///
/// let test_nu = PipeCorrelations::gnielinski_correlation_liquids(
/// Re,Pr_fluid, Pr_wall,darcy_friction_factor);
/// ///
/// approx::assert_relative_eq!(nu_f_reference, test_nu, 
/// max_relative=0.01);
///
/// ```
///
pub fn gnielinski_correlation_liquids(Re: f64, Pr_fluid: f64,
                              Pr_wall: f64,
                              darcy_friction_factor: f64) -> f64 {

    if Pr_fluid < 0.5 {
        panic!("gnielinski Pr_fluid < 0.5, too low");
    }

    if Pr_fluid > 1e5_f64 {
        panic!("gnielinski Pr_fluid > 1e5, too high");
    }

    if Pr_wall < 0.5 {
        panic!("gnielinski Pr_wall < 0.5, too low");
    }

    if Pr_wall > 1e5_f64 {
        panic!("gnielinski Pr_wall > 1e5, too high");
    }

    let prandtl_ratio: f64 = Pr_fluid/Pr_wall;

    if prandtl_ratio < 0.05 {
        panic!("gnielinski prandtl_ratio < 0.05, too low");
    }

    if prandtl_ratio > 20_f64 {
        panic!("gnielinski prandtl_ratio > 20, too high");
    }

    if Re < 2300_f64 {
        panic!("gnielinski Re < 2300, laminar or transition");
    }

    if Re > 1e6_f64 {
        panic!("gnielinski Re > 1e6, too high");
    }

    // now we start calculating
    let darcy_ratio: f64 = darcy_friction_factor/8.0;

    let numerator: f64 = darcy_ratio * (Re - 1000_f64) * Pr_fluid *
        prandtl_ratio.powf(0.11);
    let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
        (Pr_fluid.powf(0.666667) - 1.0);

    let fluid_nusselt_number = numerator/denominator;
    

    return fluid_nusselt_number;
}


/// Improved Gnielinski Equation for liquids
///
///
/// https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf
///
/// The original Gnielinski equation does not have a smooth transition with
/// the laminar region. In the following paper, an interpolation scheme
/// is proposed by gnielinski:
///
/// Gnielinski, V. (2013). On heat transfer 
/// in tubes. International Journal of Heat and 
/// Mass Transfer, 63, 134-140.
///
/// 
///
///
/// However, 
///
/// ```rust
///
/// extern crate approx;
/// use heat_transfer_rust::NusseltCorrelations::PipeCorrelations;
///
/// let Re = 8000_f64;
/// let Pr_fluid = 17_f64;
/// let Pr_wall = 12_f64;
/// let darcy_friction_factor = 0.005_f64;
///
/// // let's now calculate the nusslet number
///
/// let prandtl_ratio = Pr_fluid/Pr_wall;
///
/// let darcy_ratio: f64 = darcy_friction_factor/8.0;
///
/// let numerator: f64 = darcy_ratio * (Re - 1000_f64) * Pr_fluid *
///     prandtl_ratio.powf(0.11);
/// let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
///     (Pr_fluid.powf(2.0/3.0) - 1.0);
/// 
///
///
/// let nu_f_reference = numerator/denominator;
///
/// let test_nu = PipeCorrelations::gnielinski_correlation_liquids(
/// Re,Pr_fluid, Pr_wall,darcy_friction_factor);
/// ///
/// approx::assert_relative_eq!(nu_f_reference, test_nu, 
/// max_relative=0.01);
///
/// // now i want to test the nusselt number bordering the laminar
/// // region
///
/// let Re_laminar = 2300_f64;
///
/// let test_nu2 = PipeCorrelations::gnielinski_correlation_liquids(
/// Re_laminar,Pr_fluid, Pr_wall,darcy_friction_factor);
///
/// approx::assert_relative_eq!(0.0, test_nu2, 
/// max_relative=0.01);
/// ```
///
pub fn improved_gnielinski_correlation_liquids(Re: f64, Pr_fluid: f64,
                              Pr_wall: f64,
                              darcy_friction_factor: f64) -> f64 {

    if Pr_fluid < 0.5 {
        panic!("gnielinski Pr_fluid < 0.5, too low");
    }

    if Pr_fluid > 1e5_f64 {
        panic!("gnielinski Pr_fluid > 1e5, too high");
    }

    if Pr_wall < 0.5 {
        panic!("gnielinski Pr_wall < 0.5, too low");
    }

    if Pr_wall > 1e5_f64 {
        panic!("gnielinski Pr_wall > 1e5, too high");
    }

    let prandtl_ratio: f64 = Pr_fluid/Pr_wall;

    if prandtl_ratio < 0.05 {
        panic!("gnielinski prandtl_ratio < 0.05, too low");
    }

    if prandtl_ratio > 20_f64 {
        panic!("gnielinski prandtl_ratio > 20, too high");
    }


    // now we start calculating
    let darcy_ratio: f64 = darcy_friction_factor/8.0;

    let numerator: f64 = darcy_ratio * (Re - 1000_f64) * Pr_fluid *
        prandtl_ratio.powf(0.11);
    let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
        (Pr_fluid.powf(0.666667) - 1.0);

    let fluid_nusselt_number = numerator/denominator;
    
    panic!("not implemented");

    return fluid_nusselt_number;
}







