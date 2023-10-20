use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::length::*;

pub struct Length;

impl Values for Length {
    fn name() -> &'static str {
        "length"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("centimetres", centimetres::to_feet, feet::to_centimetres),
            conversion("feet", identity, identity),
            conversion("inches", inches::to_feet, feet::to_inches),
            conversion("kilometres", kilometres::to_feet, feet::to_kilometres),
            conversion("metres", metres::to_feet, feet::to_metres),
            conversion("miles", miles::to_feet, feet::to_miles),
            conversion("millimetres", millimetres::to_feet, feet::to_millimetres),
            conversion(
                "nautical-miles",
                nautical_miles::to_feet,
                feet::to_nautical_miles,
            ),
            conversion("yards", yards::to_feet, feet::to_yards),
        ])
    }
}
