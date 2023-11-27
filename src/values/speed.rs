use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::speed::*;

const FEET_PER_SECOND: &str = "feet-per-second";
const KILOMETRES_PER_HOUR: &str = "kilometres-per-hour";
const KNOTS: &str = "knots";
const METRES_PER_SECOND: &str = "metres-per-second";
const MILES_PER_HOUR: &str = "miles-per-hour";

pub struct Speed;

impl Category for Speed {
    fn name() -> &'static str {
        "speed"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                FEET_PER_SECOND,
                HashMap::from_iter([
                    (FEET_PER_SECOND, identity as ConversionFunction),
                    (KILOMETRES_PER_HOUR, feet_per_second::to_kilometres_per_hour),
                    (KNOTS, feet_per_second::to_knots),
                    (METRES_PER_SECOND, feet_per_second::to_metres_per_second),
                    (MILES_PER_HOUR, feet_per_second::to_miles_per_hour),
                ]),
            ),
            (
                KILOMETRES_PER_HOUR,
                HashMap::from_iter([
                    (
                        FEET_PER_SECOND,
                        kilometres_per_hour::to_feet_per_second as ConversionFunction,
                    ),
                    (KILOMETRES_PER_HOUR, identity),
                    (KNOTS, kilometres_per_hour::to_knots),
                    (METRES_PER_SECOND, kilometres_per_hour::to_metres_per_second),
                    (MILES_PER_HOUR, kilometres_per_hour::to_miles_per_hour),
                ]),
            ),
            (
                KNOTS,
                HashMap::from_iter([
                    (
                        FEET_PER_SECOND,
                        knots::to_feet_per_second as ConversionFunction,
                    ),
                    (KILOMETRES_PER_HOUR, knots::to_kilometres_per_hour),
                    (KNOTS, identity),
                    (METRES_PER_SECOND, knots::to_metres_per_second),
                    (MILES_PER_HOUR, knots::to_miles_per_hour),
                ]),
            ),
            (
                METRES_PER_SECOND,
                HashMap::from_iter([
                    (
                        FEET_PER_SECOND,
                        metres_per_second::to_feet_per_second as ConversionFunction,
                    ),
                    (
                        KILOMETRES_PER_HOUR,
                        metres_per_second::to_kilometres_per_hour,
                    ),
                    (KNOTS, metres_per_second::to_knots),
                    (METRES_PER_SECOND, identity),
                    (MILES_PER_HOUR, metres_per_second::to_miles_per_hour),
                ]),
            ),
            (
                MILES_PER_HOUR,
                HashMap::from_iter([
                    (
                        FEET_PER_SECOND,
                        miles_per_hour::to_feet_per_second as ConversionFunction,
                    ),
                    (KILOMETRES_PER_HOUR, miles_per_hour::to_kilometres_per_hour),
                    (KNOTS, miles_per_hour::to_knots),
                    (METRES_PER_SECOND, miles_per_hour::to_metres_per_second),
                    (MILES_PER_HOUR, identity),
                ]),
            ),
        ])
    }
}
