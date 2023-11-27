use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::temperature::*;

const CELSIUS: &str = "celsius";
const FAHRENHEIT: &str = "fahrenheit";
const KELVIN: &str = "kelvin";
const RANKINE: &str = "rankine";

pub struct Temperature;

impl Category for Temperature {
    fn name() -> &'static str {
        "temperature"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                CELSIUS,
                HashMap::from_iter([
                    (CELSIUS, identity as ConversionFunction),
                    (FAHRENHEIT, celsius::to_fahrenheit),
                    (KELVIN, celsius::to_kelvin),
                    (RANKINE, celsius::to_rankine),
                ]),
            ),
            (
                FAHRENHEIT,
                HashMap::from_iter([
                    (CELSIUS, fahrenheit::to_celsius as ConversionFunction),
                    (FAHRENHEIT, identity),
                    (KELVIN, fahrenheit::to_kelvin),
                    (RANKINE, fahrenheit::to_rankine),
                ]),
            ),
            (
                KELVIN,
                HashMap::from_iter([
                    (CELSIUS, kelvin::to_celsius as ConversionFunction),
                    (FAHRENHEIT, kelvin::to_fahrenheit),
                    (KELVIN, identity),
                    (RANKINE, kelvin::to_rankine),
                ]),
            ),
            (
                RANKINE,
                HashMap::from_iter([
                    (CELSIUS, rankine::to_celsius as ConversionFunction),
                    (FAHRENHEIT, rankine::to_fahrenheit),
                    (KELVIN, rankine::to_kelvin),
                    (RANKINE, identity),
                ]),
            ),
        ])
    }
}
