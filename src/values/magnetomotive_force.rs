use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::magnetomotive_force::*;

pub struct MagnetomotiveForce;

impl Values for MagnetomotiveForce {
    fn name() -> &'static str {
        "magnetomotive-force"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "ampereturns",
                HashMap::from_iter([
                    ("ampereturns", identity as ConversionFunction),
                    ("gilberts", ampereturns::to_gilberts),
                ]),
            ),
            (
                "gilberts",
                HashMap::from_iter([
                    (
                        "ampereturns",
                        gilberts::to_ampereturns as ConversionFunction,
                    ),
                    ("gilberts", identity),
                ]),
            ),
        ])
    }
}
