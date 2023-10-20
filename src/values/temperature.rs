use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::temperature::*;

pub struct Temperature;

impl Values for Temperature {
    fn name() -> &'static str {
        "temperature"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("celsius", identity, identity),
            conversion("fahrenheit", fahrenheit::to_celsius, celsius::to_fahrenheit),
            conversion("kelvin", kelvin::to_celsius, celsius::to_kelvin),
            conversion("rankine", rankine::to_celsius, celsius::to_rankine),
        ])
    }
}
