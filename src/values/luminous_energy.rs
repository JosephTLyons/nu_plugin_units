use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::luminous_energy::*;

pub struct LuminousEnergy;

impl Values for LuminousEnergy {
    fn name() -> &'static str {
        "luminous-energy"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("lumen-hour", identity, identity),
            conversion(
                "lumen-minute",
                lumen_minute::to_lumen_hour,
                lumen_hour::to_lumen_minute,
            ),
            conversion(
                "lumen-second",
                lumen_second::to_lumen_hour,
                lumen_hour::to_lumen_second,
            ),
            conversion("talbot", talbot::to_lumen_hour, lumen_hour::to_talbot),
        ])
    }
}
