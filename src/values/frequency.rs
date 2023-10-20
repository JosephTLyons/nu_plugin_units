use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::frequency::*;

pub struct Frequency;

impl Values for Frequency {
    fn name() -> &'static str {
        "frequency"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "gigahertz",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "hertz",
                hertz::to_gigahertz as BaseConversionFunction,
                gigahertz::to_hertz as BaseConversionFunction,
            ),
            conversion(
                "kilohertz",
                kilohertz::to_gigahertz,
                gigahertz::to_kilohertz,
            ),
            conversion(
                "megahertz",
                megahertz::to_gigahertz,
                gigahertz::to_megahertz,
            ),
        ])
    }
}
