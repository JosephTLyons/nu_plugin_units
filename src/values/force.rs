use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::force::*;

pub struct Force;

impl Values for Force {
    fn name() -> &'static str {
        "force"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("dynes", identity, identity),
            conversion(
                "kilogramforce",
                kilogramforce::to_dynes,
                dynes::to_kilogramforce,
            ),
            conversion("newtons", newtons::to_dynes, dynes::to_newtons),
            conversion("poundals", poundals::to_dynes, dynes::to_poundals),
        ])
    }
}
