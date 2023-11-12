use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::time::*;

pub struct Time;

impl Values for Time {
    fn name() -> &'static str {
        "time"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "centuries",
                HashMap::from_iter([
                    ("centuries", identity as ConversionFunction),
                    ("days", centuries::to_days),
                    ("decades", centuries::to_decades),
                    ("hours", centuries::to_hours),
                    ("microseconds", centuries::to_microseconds),
                    ("milliseconds", centuries::to_milliseconds),
                    ("minutes", centuries::to_minutes),
                    ("months", centuries::to_months),
                    ("nanoseconds", centuries::to_nanoseconds),
                    ("seconds", centuries::to_seconds),
                    ("weeks", centuries::to_weeks),
                    ("years", centuries::to_years),
                ]),
            ),
            (
                "days",
                HashMap::from_iter([
                    ("centuries", days::to_centuries as ConversionFunction),
                    ("days", identity),
                    ("decades", days::to_decades),
                    ("hours", days::to_hours),
                    ("microseconds", days::to_microseconds),
                    ("milliseconds", days::to_milliseconds),
                    ("minutes", days::to_minutes),
                    ("months", days::to_months),
                    ("nanoseconds", days::to_nanoseconds),
                    ("seconds", days::to_seconds),
                    ("weeks", days::to_weeks),
                    ("years", days::to_years),
                ]),
            ),
            (
                "decades",
                HashMap::from_iter([
                    ("centuries", decades::to_centuries as ConversionFunction),
                    ("days", decades::to_days),
                    ("decades", identity),
                    ("hours", decades::to_hours),
                    ("microseconds", decades::to_microseconds),
                    ("milliseconds", decades::to_milliseconds),
                    ("minutes", decades::to_minutes),
                    ("months", decades::to_months),
                    ("nanoseconds", decades::to_nanoseconds),
                    ("seconds", decades::to_seconds),
                    ("weeks", decades::to_weeks),
                    ("years", decades::to_years),
                ]),
            ),
            (
                "hours",
                HashMap::from_iter([
                    ("centuries", hours::to_centuries as ConversionFunction),
                    ("days", hours::to_days),
                    ("decades", hours::to_decades),
                    ("hours", identity),
                    ("microseconds", hours::to_microseconds),
                    ("milliseconds", hours::to_milliseconds),
                    ("minutes", hours::to_minutes),
                    ("months", hours::to_months),
                    ("nanoseconds", hours::to_nanoseconds),
                    ("seconds", hours::to_seconds),
                    ("weeks", hours::to_weeks),
                    ("years", hours::to_years),
                ]),
            ),
            (
                "microseconds",
                HashMap::from_iter([
                    (
                        "centuries",
                        microseconds::to_centuries as ConversionFunction,
                    ),
                    ("days", microseconds::to_days),
                    ("decades", microseconds::to_decades),
                    ("hours", microseconds::to_hours),
                    ("microseconds", identity),
                    ("milliseconds", microseconds::to_milliseconds),
                    ("minutes", microseconds::to_minutes),
                    ("months", microseconds::to_months),
                    ("nanoseconds", microseconds::to_nanoseconds),
                    ("seconds", microseconds::to_seconds),
                    ("weeks", microseconds::to_weeks),
                    ("years", microseconds::to_years),
                ]),
            ),
            (
                "milliseconds",
                HashMap::from_iter([
                    (
                        "centuries",
                        milliseconds::to_centuries as ConversionFunction,
                    ),
                    ("days", milliseconds::to_days),
                    ("decades", milliseconds::to_decades),
                    ("hours", milliseconds::to_hours),
                    ("microseconds", milliseconds::to_microseconds),
                    ("milliseconds", identity),
                    ("minutes", milliseconds::to_minutes),
                    ("months", milliseconds::to_months),
                    ("nanoseconds", milliseconds::to_nanoseconds),
                    ("seconds", milliseconds::to_seconds),
                    ("weeks", milliseconds::to_weeks),
                    ("years", milliseconds::to_years),
                ]),
            ),
            (
                "minutes",
                HashMap::from_iter([
                    ("centuries", minutes::to_centuries as ConversionFunction),
                    ("days", minutes::to_days),
                    ("decades", minutes::to_decades),
                    ("hours", minutes::to_hours),
                    ("microseconds", minutes::to_microseconds),
                    ("milliseconds", minutes::to_milliseconds),
                    ("minutes", identity),
                    ("months", minutes::to_months),
                    ("nanoseconds", minutes::to_nanoseconds),
                    ("seconds", minutes::to_seconds),
                    ("weeks", minutes::to_weeks),
                    ("years", minutes::to_years),
                ]),
            ),
            (
                "months",
                HashMap::from_iter([
                    ("centuries", months::to_centuries as ConversionFunction),
                    ("days", months::to_days),
                    ("decades", months::to_decades),
                    ("hours", months::to_hours),
                    ("microseconds", months::to_microseconds),
                    ("milliseconds", months::to_milliseconds),
                    ("minutes", months::to_minutes),
                    ("months", identity),
                    ("nanoseconds", months::to_nanoseconds),
                    ("seconds", months::to_seconds),
                    ("weeks", months::to_weeks),
                    ("years", months::to_years),
                ]),
            ),
            (
                "nanoseconds",
                HashMap::from_iter([
                    ("centuries", nanoseconds::to_centuries as ConversionFunction),
                    ("days", nanoseconds::to_days),
                    ("decades", nanoseconds::to_decades),
                    ("hours", nanoseconds::to_hours),
                    ("microseconds", nanoseconds::to_microseconds),
                    ("milliseconds", nanoseconds::to_milliseconds),
                    ("minutes", nanoseconds::to_minutes),
                    ("months", nanoseconds::to_months),
                    ("nanoseconds", identity),
                    ("seconds", nanoseconds::to_seconds),
                    ("weeks", nanoseconds::to_weeks),
                    ("years", nanoseconds::to_years),
                ]),
            ),
            (
                "seconds",
                HashMap::from_iter([
                    ("centuries", seconds::to_centuries as ConversionFunction),
                    ("days", seconds::to_days),
                    ("decades", seconds::to_decades),
                    ("hours", seconds::to_hours),
                    ("microseconds", seconds::to_microseconds),
                    ("milliseconds", seconds::to_milliseconds),
                    ("minutes", seconds::to_minutes),
                    ("months", seconds::to_months),
                    ("nanoseconds", seconds::to_nanoseconds),
                    ("seconds", identity),
                    ("weeks", seconds::to_weeks),
                    ("years", seconds::to_years),
                ]),
            ),
            (
                "weeks",
                HashMap::from_iter([
                    ("centuries", weeks::to_centuries as ConversionFunction),
                    ("days", weeks::to_days),
                    ("decades", weeks::to_decades),
                    ("hours", weeks::to_hours),
                    ("microseconds", weeks::to_microseconds),
                    ("milliseconds", weeks::to_milliseconds),
                    ("minutes", weeks::to_minutes),
                    ("months", weeks::to_months),
                    ("nanoseconds", weeks::to_nanoseconds),
                    ("seconds", weeks::to_seconds),
                    ("weeks", identity),
                    ("years", weeks::to_years),
                ]),
            ),
            (
                "years",
                HashMap::from_iter([
                    ("centuries", years::to_centuries as ConversionFunction),
                    ("days", years::to_days),
                    ("decades", years::to_decades),
                    ("hours", years::to_hours),
                    ("microseconds", years::to_microseconds),
                    ("milliseconds", years::to_milliseconds),
                    ("minutes", years::to_minutes),
                    ("months", years::to_months),
                    ("nanoseconds", years::to_nanoseconds),
                    ("seconds", years::to_seconds),
                    ("weeks", years::to_weeks),
                    ("years", identity),
                ]),
            ),
        ])
    }
}
