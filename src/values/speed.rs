use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::speed::*;

pub struct Speed;

impl Values for Speed {
    fn name() -> &'static str {
        "speed"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "feet-per-second",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "kilometres-per-hour",
                kilometres_per_hour::to_feet_per_second as BaseConversionFunction,
                feet_per_second::to_kilometres_per_hour as BaseConversionFunction,
            ),
            conversion(
                "knots",
                knots::to_feet_per_second as BaseConversionFunction,
                feet_per_second::to_knots as BaseConversionFunction,
            ),
            conversion(
                "metres-per-second",
                metres_per_second::to_feet_per_second as BaseConversionFunction,
                feet_per_second::to_metres_per_second as BaseConversionFunction,
            ),
            conversion(
                "miles-per-hour",
                miles_per_hour::to_feet_per_second as BaseConversionFunction,
                feet_per_second::to_miles_per_hour as BaseConversionFunction,
            ),
        ])
    }
}
