use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::area::*;

const ACRES: &str = "acres";
const HECTARES: &str = "hectares";
const SQUARE_FEET: &str = "square-feet";
const SQUARE_INCHES: &str = "square-inches";
const SQUARE_KILOMETRES: &str = "square-kilometres";
const SQUARE_METRES: &str = "square-metres";
const SQUARE_MILES: &str = "square-miles";

pub struct Area;

impl Category for Area {
    fn name() -> &'static str {
        "area"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                ACRES,
                HashMap::from_iter([
                    (ACRES, identity as ConversionFunction),
                    (HECTARES, acres::to_hectares),
                    (SQUARE_FEET, acres::to_square_feet),
                    (SQUARE_INCHES, acres::to_square_inches),
                    (SQUARE_KILOMETRES, acres::to_square_kilometres),
                    (SQUARE_METRES, acres::to_square_metres),
                    (SQUARE_MILES, acres::to_square_miles),
                ]),
            ),
            (
                HECTARES,
                HashMap::from_iter([
                    (ACRES, hectares::to_acres as ConversionFunction),
                    (HECTARES, identity),
                    (SQUARE_FEET, hectares::to_square_feet),
                    (SQUARE_INCHES, hectares::to_square_inches),
                    (SQUARE_KILOMETRES, hectares::to_square_kilometres),
                    (SQUARE_METRES, hectares::to_square_metres),
                    (SQUARE_MILES, hectares::to_square_miles),
                ]),
            ),
            (
                SQUARE_FEET,
                HashMap::from_iter([
                    (ACRES, square_feet::to_acres as ConversionFunction),
                    (HECTARES, square_feet::to_hectares),
                    (SQUARE_FEET, identity),
                    (SQUARE_INCHES, square_feet::to_square_inches),
                    (SQUARE_KILOMETRES, square_feet::to_square_kilometres),
                    (SQUARE_METRES, square_feet::to_square_metres),
                    (SQUARE_MILES, square_feet::to_square_miles),
                ]),
            ),
            (
                SQUARE_INCHES,
                HashMap::from_iter([
                    (ACRES, square_inches::to_acres as ConversionFunction),
                    (HECTARES, square_inches::to_hectares),
                    (SQUARE_FEET, square_inches::to_square_feet),
                    (SQUARE_INCHES, identity),
                    (SQUARE_KILOMETRES, square_inches::to_square_kilometres),
                    (SQUARE_METRES, square_inches::to_square_metres),
                    (SQUARE_MILES, square_inches::to_square_miles),
                ]),
            ),
            (
                SQUARE_KILOMETRES,
                HashMap::from_iter([
                    (ACRES, square_kilometres::to_acres as ConversionFunction),
                    (HECTARES, square_kilometres::to_hectares),
                    (SQUARE_FEET, square_kilometres::to_square_feet),
                    (SQUARE_INCHES, square_kilometres::to_square_inches),
                    (SQUARE_KILOMETRES, identity),
                    (SQUARE_METRES, square_kilometres::to_square_metres),
                    (SQUARE_MILES, square_kilometres::to_square_miles),
                ]),
            ),
            (
                SQUARE_METRES,
                HashMap::from_iter([
                    (ACRES, square_metres::to_acres as ConversionFunction),
                    (HECTARES, square_metres::to_hectares),
                    (SQUARE_FEET, square_metres::to_square_feet),
                    (SQUARE_INCHES, square_metres::to_square_inches),
                    (SQUARE_KILOMETRES, square_metres::to_square_kilometres),
                    (SQUARE_METRES, identity),
                    (SQUARE_MILES, square_metres::to_square_miles),
                ]),
            ),
            (
                SQUARE_MILES,
                HashMap::from_iter([
                    (ACRES, square_miles::to_acres as ConversionFunction),
                    (HECTARES, square_miles::to_hectares),
                    (SQUARE_FEET, square_miles::to_square_feet),
                    (SQUARE_INCHES, square_miles::to_square_inches),
                    (SQUARE_KILOMETRES, square_miles::to_square_kilometres),
                    (SQUARE_METRES, square_miles::to_square_metres),
                    (SQUARE_MILES, identity),
                ]),
            ),
        ])
    }
}
