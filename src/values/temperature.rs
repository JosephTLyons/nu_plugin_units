use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::temperature::*;

pub struct Temperature;

impl Values for Temperature {
    fn name() -> &'static str {
        "temperature"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "celsius",
                HashMap::from_iter([
                    ("celsius", identity as ConversionFunction),
                    ("fahrenheit", celsius::to_fahrenheit),
                    ("kelvin", celsius::to_kelvin),
                    ("rankine", celsius::to_rankine),
                ]),
            ),
            (
                "fahrenheit",
                HashMap::from_iter([
                    ("celsius", fahrenheit::to_celsius as ConversionFunction),
                    ("fahrenheit", identity),
                    ("kelvin", fahrenheit::to_kelvin),
                    ("rankine", fahrenheit::to_rankine),
                ]),
            ),
            (
                "kelvin",
                HashMap::from_iter([
                    ("celsius", kelvin::to_celsius as ConversionFunction),
                    ("fahrenheit", kelvin::to_fahrenheit),
                    ("kelvin", identity),
                    ("rankine", kelvin::to_rankine),
                ]),
            ),
            (
                "rankine",
                HashMap::from_iter([
                    ("celsius", rankine::to_celsius as ConversionFunction),
                    ("fahrenheit", rankine::to_fahrenheit),
                    ("kelvin", rankine::to_kelvin),
                    ("rankine", identity),
                ]),
            ),
        ])
    }
}
