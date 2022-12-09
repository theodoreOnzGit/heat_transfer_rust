//! this module contains functions to calculate the enthalpy
//! of a control volume at the the next timestep
//!
//!
//! For Control Volume calculations in general
//! we have the form:
//!
//! dH_cv/dt = H_in - H_out + Q_s + W_s
//!
//! H_cv is the control volume enthalpy
//!
//! H_in is the sum of enthalpy flows in
//!
//! H_out is the sum of enthalpy flows out
//!
//! Q_s is the heat supplied to the volume per second
//!
//! W_s is the work done on the system per second
//!
//!
//! After discretisation, we can use:
//!
//! (H_cv (t+1) - H_cv (t)) / dt = H_in - H_out + Q_s + W_s
//!
//! H_cv (t+1)  = dt * (H_in - H_out + Q_s + W_s) + H_cv (t)
//!
//! It remains to be seen whether the enthalpy flows in and
//! out are calculated at the current time step  (explicit)
//! or next time step (implict)
//!
//! Of course, implicit calculations are more stable but
//! slower in general than explicit calculations
//!
//! we will be using the uom module to ensure that calculations are
//! done with correct units



extern crate uom;
use uom::si::f64::*;

/// This module contains functions which help to calculate
/// the enthalpy explicitly, ie using enthalpy in and out for current
/// timestep
///
/// The known information from which to start is 
/// (1) the mass flow rate
/// (2) the temperatures of each part of the fluid
/// (3) mass of the control volume
#[allow(non_snake_case)]
pub mod ExplictCalculations;


/// This module contains commonly used functions for Explicit and
/// Implicit timestep calculations
#[allow(non_snake_case)]
pub mod CommonFunctions;

