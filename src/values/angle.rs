use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::angle::*;

const DEGREES: &str = "degrees";
const GRADIANS: &str = "gradians";
const MILLIRADIANS: &str = "milliradians";
const MINUTE_OF_ARC: &str = "minute-of-arc";
const RADIANS: &str = "radians";
const SECONDS_OF_ARC: &str = "seconds-of-arc";

pub struct Angle;

impl Category for Angle {
    fn name() -> &'static str {
        "angle"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                DEGREES,
                HashMap::from_iter([
                    (DEGREES, identity as ConversionFunction),
                    (GRADIANS, degrees::to_gradians),
                    (MILLIRADIANS, degrees::to_milliradians),
                    (MINUTE_OF_ARC, degrees::to_minute_of_arc),
                    (RADIANS, degrees::to_radians),
                    (SECONDS_OF_ARC, degrees::to_seconds_of_arc),
                ]),
            ),
            (
                GRADIANS,
                HashMap::from_iter([
                    (DEGREES, gradians::to_degrees as ConversionFunction),
                    (GRADIANS, identity),
                    (MILLIRADIANS, gradians::to_milliradians),
                    (MINUTE_OF_ARC, gradians::to_minute_of_arc),
                    (RADIANS, gradians::to_radians),
                    (SECONDS_OF_ARC, gradians::to_seconds_of_arc),
                ]),
            ),
            (
                MILLIRADIANS,
                HashMap::from_iter([
                    (DEGREES, milliradians::to_degrees as ConversionFunction),
                    (GRADIANS, milliradians::to_gradians),
                    (MILLIRADIANS, identity),
                    (MINUTE_OF_ARC, milliradians::to_minute_of_arc),
                    (RADIANS, milliradians::to_radians),
                    (SECONDS_OF_ARC, milliradians::to_seconds_of_arc),
                ]),
            ),
            (
                MINUTE_OF_ARC,
                HashMap::from_iter([
                    (DEGREES, minute_of_arc::to_degrees as ConversionFunction),
                    (GRADIANS, minute_of_arc::to_gradians),
                    (MILLIRADIANS, minute_of_arc::to_milliradians),
                    (MINUTE_OF_ARC, identity),
                    (RADIANS, minute_of_arc::to_radians),
                    (SECONDS_OF_ARC, minute_of_arc::to_seconds_of_arc),
                ]),
            ),
            (
                RADIANS,
                HashMap::from_iter([
                    (DEGREES, radians::to_degrees as ConversionFunction),
                    (GRADIANS, radians::to_gradians),
                    (MILLIRADIANS, radians::to_milliradians),
                    (MINUTE_OF_ARC, radians::to_minute_of_arc),
                    (RADIANS, identity),
                    (SECONDS_OF_ARC, radians::to_seconds_of_arc),
                ]),
            ),
            (
                SECONDS_OF_ARC,
                HashMap::from_iter([
                    (DEGREES, seconds_of_arc::to_degrees as ConversionFunction),
                    (GRADIANS, seconds_of_arc::to_gradians),
                    (MILLIRADIANS, seconds_of_arc::to_milliradians),
                    (MINUTE_OF_ARC, seconds_of_arc::to_minute_of_arc),
                    (RADIANS, seconds_of_arc::to_radians),
                    (SECONDS_OF_ARC, identity),
                ]),
            ),
        ])
    }
}
