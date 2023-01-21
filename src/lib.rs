#![warn(missing_docs)]


pub mod NusseltCorrelations;
pub mod ControlVolumeCalculations;

#[macro_use]
extern crate uom;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
