use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::time::*;

pub struct Time;

impl Values for Time {
    fn name() -> &'static str {
        "time"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("centuries", identity, identity),
            conversion("days", days::to_centuries, centuries::to_days),
            conversion("decades", decades::to_centuries, centuries::to_decades),
            conversion("hours", hours::to_centuries, centuries::to_hours),
            conversion(
                "microseconds",
                microseconds::to_centuries,
                centuries::to_microseconds,
            ),
            conversion(
                "milliseconds",
                milliseconds::to_centuries,
                centuries::to_milliseconds,
            ),
            conversion("minutes", minutes::to_centuries, centuries::to_minutes),
            conversion("months", months::to_centuries, centuries::to_months),
            conversion(
                "nanoseconds",
                nanoseconds::to_centuries,
                centuries::to_nanoseconds,
            ),
            conversion("seconds", seconds::to_centuries, centuries::to_seconds),
            conversion("weeks", weeks::to_centuries, centuries::to_weeks),
            conversion("years", years::to_centuries, centuries::to_years),
        ])
    }
}
