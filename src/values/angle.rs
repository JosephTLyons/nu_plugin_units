use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::angle::*;

// TODO - pull all of the strings into constants, or enums, in all files

pub struct Angle;

impl Values for Angle {
    fn name() -> &'static str {
        "angle"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "degrees",
                HashMap::from_iter([
                    ("degrees", identity as ConversionFunction),
                    ("gradians", degrees::to_gradians),
                    ("milliradians", degrees::to_milliradians),
                    ("minute-of-arc", degrees::to_minute_of_arc),
                    ("radians", degrees::to_radians),
                    ("seconds-of-arc", degrees::to_seconds_of_arc),
                ]),
            ),
            (
                "gradians",
                HashMap::from_iter([
                    ("degrees", gradians::to_degrees as ConversionFunction),
                    ("gradians", identity),
                    ("milliradians", gradians::to_milliradians),
                    ("minute-of-arc", gradians::to_minute_of_arc),
                    ("radians", gradians::to_radians),
                    ("seconds-of-arc", gradians::to_seconds_of_arc),
                ]),
            ),
            (
                "milliradians",
                HashMap::from_iter([
                    ("degrees", milliradians::to_degrees as ConversionFunction),
                    ("gradians", milliradians::to_gradians),
                    ("milliradians", identity),
                    ("minute-of-arc", milliradians::to_minute_of_arc),
                    ("radians", milliradians::to_radians),
                    ("seconds-of-arc", milliradians::to_seconds_of_arc),
                ]),
            ),
            (
                "minute-of-arc",
                HashMap::from_iter([
                    ("degrees", minute_of_arc::to_degrees as ConversionFunction),
                    ("gradians", minute_of_arc::to_gradians),
                    ("milliradians", minute_of_arc::to_milliradians),
                    ("minute-of-arc", identity),
                    ("radians", minute_of_arc::to_radians),
                    ("seconds-of-arc", minute_of_arc::to_seconds_of_arc),
                ]),
            ),
            (
                "radians",
                HashMap::from_iter([
                    ("degrees", radians::to_degrees as ConversionFunction),
                    ("gradians", radians::to_gradians),
                    ("milliradians", radians::to_milliradians),
                    ("minute-of-arc", radians::to_minute_of_arc),
                    ("radians", identity),
                    ("seconds-of-arc", radians::to_seconds_of_arc),
                ]),
            ),
            (
                "seconds-of-arc",
                HashMap::from_iter([
                    ("degrees", seconds_of_arc::to_degrees as ConversionFunction),
                    ("gradians", seconds_of_arc::to_gradians),
                    ("milliradians", seconds_of_arc::to_milliradians),
                    ("minute-of-arc", seconds_of_arc::to_minute_of_arc),
                    ("radians", seconds_of_arc::to_radians),
                    ("seconds-of-arc", identity),
                ]),
            ),
        ])
    }
}
