use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::angle::*;

pub struct Angle;

impl Values for Angle {
    fn name() -> &'static str {
        "angle"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "degrees",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion("gradians", gradians::to_degrees, degrees::to_gradians),
            conversion(
                "milliradians",
                milliradians::to_degrees,
                degrees::to_milliradians,
            ),
            conversion(
                "minute-of-arc",
                minute_of_arc::to_degrees,
                degrees::to_minute_of_arc,
            ),
            conversion("radians", radians::to_degrees, degrees::to_radians),
            conversion(
                "seconds-of-arc",
                seconds_of_arc::to_degrees,
                degrees::to_seconds_of_arc,
            ),
        ])
    }
}
