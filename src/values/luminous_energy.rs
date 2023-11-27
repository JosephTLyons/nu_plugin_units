use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::luminous_energy::*;

const LUMEN_HOURS: &str = "lumen-hours";
const LUMEN_MINUTES: &str = "lumen-minutes";
const LUMEN_SECONDS: &str = "lumen-seconds";
const TALBOTS: &str = "talbots";

pub struct LuminousEnergy;

impl Category for LuminousEnergy {
    fn name() -> &'static str {
        "luminous-energy"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                LUMEN_HOURS,
                HashMap::from_iter([
                    (LUMEN_HOURS, identity as ConversionFunction),
                    (LUMEN_MINUTES, lumen_hour::to_lumen_minute),
                    (LUMEN_SECONDS, lumen_hour::to_lumen_second),
                    (TALBOTS, lumen_hour::to_talbot),
                ]),
            ),
            (
                LUMEN_MINUTES,
                HashMap::from_iter([
                    (
                        LUMEN_HOURS,
                        lumen_minute::to_lumen_hour as ConversionFunction,
                    ),
                    (LUMEN_MINUTES, identity),
                    (LUMEN_SECONDS, lumen_minute::to_lumen_second),
                    (TALBOTS, lumen_minute::to_talbot),
                ]),
            ),
            (
                LUMEN_SECONDS,
                HashMap::from_iter([
                    (
                        LUMEN_HOURS,
                        lumen_second::to_lumen_hour as ConversionFunction,
                    ),
                    (LUMEN_MINUTES, lumen_second::to_lumen_minute),
                    (LUMEN_SECONDS, identity),
                    (TALBOTS, lumen_second::to_talbot),
                ]),
            ),
            (
                TALBOTS,
                HashMap::from_iter([
                    (LUMEN_HOURS, talbot::to_lumen_hour as ConversionFunction),
                    (LUMEN_MINUTES, talbot::to_lumen_minute),
                    (LUMEN_SECONDS, talbot::to_lumen_second),
                    (TALBOTS, identity),
                ]),
            ),
        ])
    }
}
