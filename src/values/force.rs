use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::force::*;

pub struct Force;

impl Values for Force {
    fn name() -> &'static str {
        "force"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "dynes",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "kilogramforce",
                kilogramforce::to_dynes as BaseConversionFunction,
                dynes::to_kilogramforce as BaseConversionFunction,
            ),
            conversion("newtons", newtons::to_dynes, dynes::to_newtons),
            conversion("poundals", poundals::to_dynes, dynes::to_poundals),
        ])
    }
}
