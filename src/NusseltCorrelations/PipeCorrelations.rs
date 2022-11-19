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
/// 
///
pub fn dittus_boelter_correlation(Re: f64, Pr: f64) -> f64 {
    panic!("not implemented");
    return 0.0;
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
///
///
/// meant for turbulent flow
pub fn sieder_tate_correlation(Re: f64, Pr: f64) -> f64 {
    panic!("not implemented");
    return 0.0;
}

/// Gnielinski Equation
///
///
/// https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf
///
/// turbulent flow, all kinds of tubes
///
pub fn gnielinski_correlation(Re: f64, Pr: f64) -> f64 {
    panic!("not implemented");
    return 0.0;
}
