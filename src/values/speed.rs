use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::speed::*;

pub struct Speed;

impl Values for Speed {
    fn name() -> &'static str {
        "speed"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "feet-per-second",
                HashMap::from_iter([
                    ("feet-per-second", identity as ConversionFunction),
                    (
                        "kilometres-per-hour",
                        feet_per_second::to_kilometres_per_hour,
                    ),
                    ("knots", feet_per_second::to_knots),
                    ("metres-per-second", feet_per_second::to_metres_per_second),
                    ("miles-per-hour", feet_per_second::to_miles_per_hour),
                ]),
            ),
            (
                "kilometres-per-hour",
                HashMap::from_iter([
                    (
                        "feet-per-second",
                        kilometres_per_hour::to_feet_per_second as ConversionFunction,
                    ),
                    ("kilometres-per-hour", identity),
                    ("knots", kilometres_per_hour::to_knots),
                    (
                        "metres-per-second",
                        kilometres_per_hour::to_metres_per_second,
                    ),
                    ("miles-per-hour", kilometres_per_hour::to_miles_per_hour),
                ]),
            ),
            (
                "knots",
                HashMap::from_iter([
                    (
                        "feet-per-second",
                        knots::to_feet_per_second as ConversionFunction,
                    ),
                    ("kilometres-per-hour", knots::to_kilometres_per_hour),
                    ("knots", identity),
                    ("metres-per-second", knots::to_metres_per_second),
                    ("miles-per-hour", knots::to_miles_per_hour),
                ]),
            ),
            (
                "metres-per-second",
                HashMap::from_iter([
                    (
                        "feet-per-second",
                        metres_per_second::to_feet_per_second as ConversionFunction,
                    ),
                    (
                        "kilometres-per-hour",
                        metres_per_second::to_kilometres_per_hour,
                    ),
                    ("knots", metres_per_second::to_knots),
                    ("metres-per-second", identity),
                    ("miles-per-hour", metres_per_second::to_miles_per_hour),
                ]),
            ),
            (
                "miles-per-hour",
                HashMap::from_iter([
                    (
                        "feet-per-second",
                        miles_per_hour::to_feet_per_second as ConversionFunction,
                    ),
                    (
                        "kilometres-per-hour",
                        miles_per_hour::to_kilometres_per_hour,
                    ),
                    ("knots", miles_per_hour::to_knots),
                    ("metres-per-second", miles_per_hour::to_metres_per_second),
                    ("miles-per-hour", identity),
                ]),
            ),
        ])
    }
}
