use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::time::*;

const CENTURIES: &str = "centuries";
const DAYS: &str = "days";
const DECADES: &str = "decades";
const HOURS: &str = "hours";
const MICROSECONDS: &str = "microseconds";
const MILLISECONDS: &str = "milliseconds";
const MINUTES: &str = "minutes";
const MONTHS: &str = "months";
const NANOSECONDS: &str = "nanoseconds";
const SECONDS: &str = "seconds";
const WEEKS: &str = "weeks";
const YEARS: &str = "years";

pub struct Time;

impl Category for Time {
    fn name() -> &'static str {
        "time"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                CENTURIES,
                HashMap::from_iter([
                    (CENTURIES, identity as ConversionFunction),
                    (DAYS, centuries::to_days),
                    (DECADES, centuries::to_decades),
                    (HOURS, centuries::to_hours),
                    (MICROSECONDS, centuries::to_microseconds),
                    (MILLISECONDS, centuries::to_milliseconds),
                    (MINUTES, centuries::to_minutes),
                    (MONTHS, centuries::to_months),
                    (NANOSECONDS, centuries::to_nanoseconds),
                    (SECONDS, centuries::to_seconds),
                    (WEEKS, centuries::to_weeks),
                    (YEARS, centuries::to_years),
                ]),
            ),
            (
                DAYS,
                HashMap::from_iter([
                    (CENTURIES, days::to_centuries as ConversionFunction),
                    (DAYS, identity),
                    (DECADES, days::to_decades),
                    (HOURS, days::to_hours),
                    (MICROSECONDS, days::to_microseconds),
                    (MILLISECONDS, days::to_milliseconds),
                    (MINUTES, days::to_minutes),
                    (MONTHS, days::to_months),
                    (NANOSECONDS, days::to_nanoseconds),
                    (SECONDS, days::to_seconds),
                    (WEEKS, days::to_weeks),
                    (YEARS, days::to_years),
                ]),
            ),
            (
                DECADES,
                HashMap::from_iter([
                    (CENTURIES, decades::to_centuries as ConversionFunction),
                    (DAYS, decades::to_days),
                    (DECADES, identity),
                    (HOURS, decades::to_hours),
                    (MICROSECONDS, decades::to_microseconds),
                    (MILLISECONDS, decades::to_milliseconds),
                    (MINUTES, decades::to_minutes),
                    (MONTHS, decades::to_months),
                    (NANOSECONDS, decades::to_nanoseconds),
                    (SECONDS, decades::to_seconds),
                    (WEEKS, decades::to_weeks),
                    (YEARS, decades::to_years),
                ]),
            ),
            (
                HOURS,
                HashMap::from_iter([
                    (CENTURIES, hours::to_centuries as ConversionFunction),
                    (DAYS, hours::to_days),
                    (DECADES, hours::to_decades),
                    (HOURS, identity),
                    (MICROSECONDS, hours::to_microseconds),
                    (MILLISECONDS, hours::to_milliseconds),
                    (MINUTES, hours::to_minutes),
                    (MONTHS, hours::to_months),
                    (NANOSECONDS, hours::to_nanoseconds),
                    (SECONDS, hours::to_seconds),
                    (WEEKS, hours::to_weeks),
                    (YEARS, hours::to_years),
                ]),
            ),
            (
                MICROSECONDS,
                HashMap::from_iter([
                    (CENTURIES, microseconds::to_centuries as ConversionFunction),
                    (DAYS, microseconds::to_days),
                    (DECADES, microseconds::to_decades),
                    (HOURS, microseconds::to_hours),
                    (MICROSECONDS, identity),
                    (MILLISECONDS, microseconds::to_milliseconds),
                    (MINUTES, microseconds::to_minutes),
                    (MONTHS, microseconds::to_months),
                    (NANOSECONDS, microseconds::to_nanoseconds),
                    (SECONDS, microseconds::to_seconds),
                    (WEEKS, microseconds::to_weeks),
                    (YEARS, microseconds::to_years),
                ]),
            ),
            (
                MILLISECONDS,
                HashMap::from_iter([
                    (CENTURIES, milliseconds::to_centuries as ConversionFunction),
                    (DAYS, milliseconds::to_days),
                    (DECADES, milliseconds::to_decades),
                    (HOURS, milliseconds::to_hours),
                    (MICROSECONDS, milliseconds::to_microseconds),
                    (MILLISECONDS, identity),
                    (MINUTES, milliseconds::to_minutes),
                    (MONTHS, milliseconds::to_months),
                    (NANOSECONDS, milliseconds::to_nanoseconds),
                    (SECONDS, milliseconds::to_seconds),
                    (WEEKS, milliseconds::to_weeks),
                    (YEARS, milliseconds::to_years),
                ]),
            ),
            (
                MINUTES,
                HashMap::from_iter([
                    (CENTURIES, minutes::to_centuries as ConversionFunction),
                    (DAYS, minutes::to_days),
                    (DECADES, minutes::to_decades),
                    (HOURS, minutes::to_hours),
                    (MICROSECONDS, minutes::to_microseconds),
                    (MILLISECONDS, minutes::to_milliseconds),
                    (MINUTES, identity),
                    (MONTHS, minutes::to_months),
                    (NANOSECONDS, minutes::to_nanoseconds),
                    (SECONDS, minutes::to_seconds),
                    (WEEKS, minutes::to_weeks),
                    (YEARS, minutes::to_years),
                ]),
            ),
            (
                MONTHS,
                HashMap::from_iter([
                    (CENTURIES, months::to_centuries as ConversionFunction),
                    (DAYS, months::to_days),
                    (DECADES, months::to_decades),
                    (HOURS, months::to_hours),
                    (MICROSECONDS, months::to_microseconds),
                    (MILLISECONDS, months::to_milliseconds),
                    (MINUTES, months::to_minutes),
                    (MONTHS, identity),
                    (NANOSECONDS, months::to_nanoseconds),
                    (SECONDS, months::to_seconds),
                    (WEEKS, months::to_weeks),
                    (YEARS, months::to_years),
                ]),
            ),
            (
                NANOSECONDS,
                HashMap::from_iter([
                    (CENTURIES, nanoseconds::to_centuries as ConversionFunction),
                    (DAYS, nanoseconds::to_days),
                    (DECADES, nanoseconds::to_decades),
                    (HOURS, nanoseconds::to_hours),
                    (MICROSECONDS, nanoseconds::to_microseconds),
                    (MILLISECONDS, nanoseconds::to_milliseconds),
                    (MINUTES, nanoseconds::to_minutes),
                    (MONTHS, nanoseconds::to_months),
                    (NANOSECONDS, identity),
                    (SECONDS, nanoseconds::to_seconds),
                    (WEEKS, nanoseconds::to_weeks),
                    (YEARS, nanoseconds::to_years),
                ]),
            ),
            (
                SECONDS,
                HashMap::from_iter([
                    (CENTURIES, seconds::to_centuries as ConversionFunction),
                    (DAYS, seconds::to_days),
                    (DECADES, seconds::to_decades),
                    (HOURS, seconds::to_hours),
                    (MICROSECONDS, seconds::to_microseconds),
                    (MILLISECONDS, seconds::to_milliseconds),
                    (MINUTES, seconds::to_minutes),
                    (MONTHS, seconds::to_months),
                    (NANOSECONDS, seconds::to_nanoseconds),
                    (SECONDS, identity),
                    (WEEKS, seconds::to_weeks),
                    (YEARS, seconds::to_years),
                ]),
            ),
            (
                WEEKS,
                HashMap::from_iter([
                    (CENTURIES, weeks::to_centuries as ConversionFunction),
                    (DAYS, weeks::to_days),
                    (DECADES, weeks::to_decades),
                    (HOURS, weeks::to_hours),
                    (MICROSECONDS, weeks::to_microseconds),
                    (MILLISECONDS, weeks::to_milliseconds),
                    (MINUTES, weeks::to_minutes),
                    (MONTHS, weeks::to_months),
                    (NANOSECONDS, weeks::to_nanoseconds),
                    (SECONDS, weeks::to_seconds),
                    (WEEKS, identity),
                    (YEARS, weeks::to_years),
                ]),
            ),
            (
                YEARS,
                HashMap::from_iter([
                    (CENTURIES, years::to_centuries as ConversionFunction),
                    (DAYS, years::to_days),
                    (DECADES, years::to_decades),
                    (HOURS, years::to_hours),
                    (MICROSECONDS, years::to_microseconds),
                    (MILLISECONDS, years::to_milliseconds),
                    (MINUTES, years::to_minutes),
                    (MONTHS, years::to_months),
                    (NANOSECONDS, years::to_nanoseconds),
                    (SECONDS, years::to_seconds),
                    (WEEKS, years::to_weeks),
                    (YEARS, identity),
                ]),
            ),
        ])
    }
}
