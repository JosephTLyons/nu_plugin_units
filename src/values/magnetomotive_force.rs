use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::magnetomotive_force::*;

pub struct MagnetomotiveForce;

impl Values for MagnetomotiveForce {
    fn name() -> &'static str {
        "magnetomotive-force"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "ampereturns",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "gilberts",
                gilberts::to_ampereturns as BaseConversionFunction,
                ampereturns::to_gilberts as BaseConversionFunction,
            ),
        ])
    }
}
