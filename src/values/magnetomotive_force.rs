use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::magnetomotive_force::*;

const AMPERETURNS: &str = "ampereturns";
const GILBERTS: &str = "gilberts";

pub struct MagnetomotiveForce;

impl Category for MagnetomotiveForce {
    fn name() -> &'static str {
        "magnetomotive-force"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                AMPERETURNS,
                HashMap::from_iter([
                    (AMPERETURNS, identity as ConversionFunction),
                    (GILBERTS, ampereturns::to_gilberts),
                ]),
            ),
            (
                GILBERTS,
                HashMap::from_iter([
                    (AMPERETURNS, gilberts::to_ampereturns as ConversionFunction),
                    (GILBERTS, identity),
                ]),
            ),
        ])
    }
}
