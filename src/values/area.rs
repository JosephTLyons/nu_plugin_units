use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::area::*;

pub struct Area;

impl Values for Area {
    fn name() -> &'static str {
        "area"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "acres",
                acres::to_square_feet as BaseConversionFunction,
                square_feet::to_acres as BaseConversionFunction,
            ),
            conversion(
                "hectares",
                hectares::to_square_feet,
                square_feet::to_hectares,
            ),
            conversion("square-feet", identity, identity),
            conversion(
                "square-inches",
                square_inches::to_square_feet,
                square_feet::to_square_inches,
            ),
            conversion(
                "square-kilometres",
                square_kilometres::to_square_feet,
                square_feet::to_square_kilometres,
            ),
            conversion(
                "square-metres",
                square_metres::to_square_feet,
                square_feet::to_square_metres,
            ),
            conversion(
                "square-miles",
                square_miles::to_square_feet,
                square_feet::to_square_miles,
            ),
        ])
    }
}
