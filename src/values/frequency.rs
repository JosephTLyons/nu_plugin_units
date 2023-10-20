use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::frequency::*;

pub struct Frequency;

impl Values for Frequency {
    fn name() -> &'static str {
        "frequency"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("gigahertz", identity, identity),
            conversion("hertz", hertz::to_gigahertz, gigahertz::to_hertz),
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
