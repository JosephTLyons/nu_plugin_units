use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::luminous_energy::*;

const LUMEN_HOUR: &str = "lumen-hour";
const LUMEN_MINUTE: &str = "lumen-minute";
const LUMEN_SECOND: &str = "lumen-second";
const TALBOT: &str = "talbot";

pub struct LuminousEnergy;

impl Values for LuminousEnergy {
    fn name() -> &'static str {
        "luminous-energy"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                LUMEN_HOUR,
                HashMap::from_iter([
                    (LUMEN_HOUR, identity as ConversionFunction),
                    (LUMEN_MINUTE, lumen_hour::to_lumen_minute),
                    (LUMEN_SECOND, lumen_hour::to_lumen_second),
                    (TALBOT, lumen_hour::to_talbot),
                ]),
            ),
            (
                LUMEN_MINUTE,
                HashMap::from_iter([
                    (
                        LUMEN_HOUR,
                        lumen_minute::to_lumen_hour as ConversionFunction,
                    ),
                    (LUMEN_MINUTE, identity),
                    (LUMEN_SECOND, lumen_minute::to_lumen_second),
                    (TALBOT, lumen_minute::to_talbot),
                ]),
            ),
            (
                LUMEN_SECOND,
                HashMap::from_iter([
                    (
                        LUMEN_HOUR,
                        lumen_second::to_lumen_hour as ConversionFunction,
                    ),
                    (LUMEN_MINUTE, lumen_second::to_lumen_minute),
                    (LUMEN_SECOND, identity),
                    (TALBOT, lumen_second::to_talbot),
                ]),
            ),
            (
                TALBOT,
                HashMap::from_iter([
                    (LUMEN_HOUR, talbot::to_lumen_hour as ConversionFunction),
                    (LUMEN_MINUTE, talbot::to_lumen_minute),
                    (LUMEN_SECOND, talbot::to_lumen_second),
                    (TALBOT, identity),
                ]),
            ),
        ])
    }
}
