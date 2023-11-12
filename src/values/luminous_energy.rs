use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::luminous_energy::*;

pub struct LuminousEnergy;

impl Values for LuminousEnergy {
    fn name() -> &'static str {
        "luminous-energy"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "lumen-hour",
                HashMap::from_iter([
                    ("lumen-hour", identity as ConversionFunction),
                    ("lumen-minute", lumen_hour::to_lumen_minute),
                    ("lumen-second", lumen_hour::to_lumen_second),
                    ("talbot", lumen_hour::to_talbot),
                ]),
            ),
            (
                "lumen-minute",
                HashMap::from_iter([
                    (
                        "lumen-hour",
                        lumen_minute::to_lumen_hour as ConversionFunction,
                    ),
                    ("lumen-minute", identity),
                    ("lumen-second", lumen_minute::to_lumen_second),
                    ("talbot", lumen_minute::to_talbot),
                ]),
            ),
            (
                "lumen-second",
                HashMap::from_iter([
                    (
                        "lumen-hour",
                        lumen_second::to_lumen_hour as ConversionFunction,
                    ),
                    ("lumen-minute", lumen_second::to_lumen_minute),
                    ("lumen-second", identity),
                    ("talbot", lumen_second::to_talbot),
                ]),
            ),
            (
                "talbot",
                HashMap::from_iter([
                    ("lumen-hour", talbot::to_lumen_hour as ConversionFunction),
                    ("lumen-minute", talbot::to_lumen_minute),
                    ("lumen-second", talbot::to_lumen_second),
                    ("talbot", identity),
                ]),
            ),
        ])
    }
}
