use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::length::*;

const CENTIMETRES: &str = "centimetres";
const FEET: &str = "feet";
const INCHES: &str = "inches";
const KILOMETRES: &str = "kilometres";
const METRES: &str = "metres";
const MILES: &str = "miles";
const MILLIMETRES: &str = "millimetres";
const NAUTICAL_MILES: &str = "nautical-miles";
const YARDS: &str = "yards";

pub struct Length;

impl Category for Length {
    fn name() -> &'static str {
        "length"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                CENTIMETRES,
                HashMap::from_iter([
                    (CENTIMETRES, identity as ConversionFunction),
                    (FEET, centimetres::to_feet),
                    (INCHES, centimetres::to_inches),
                    (KILOMETRES, centimetres::to_kilometres),
                    (METRES, centimetres::to_metres),
                    (MILES, centimetres::to_miles),
                    (MILLIMETRES, centimetres::to_millimetres),
                    (NAUTICAL_MILES, centimetres::to_nautical_miles),
                    (YARDS, centimetres::to_yards),
                ]),
            ),
            (
                FEET,
                HashMap::from_iter([
                    (CENTIMETRES, feet::to_centimetres as ConversionFunction),
                    (FEET, identity),
                    (INCHES, feet::to_inches),
                    (KILOMETRES, feet::to_kilometres),
                    (METRES, feet::to_metres),
                    (MILES, feet::to_miles),
                    (MILLIMETRES, feet::to_millimetres),
                    (NAUTICAL_MILES, feet::to_nautical_miles),
                    (YARDS, feet::to_yards),
                ]),
            ),
            (
                INCHES,
                HashMap::from_iter([
                    (CENTIMETRES, inches::to_centimetres as ConversionFunction),
                    (FEET, inches::to_feet),
                    (INCHES, identity),
                    (KILOMETRES, inches::to_kilometres),
                    (METRES, inches::to_metres),
                    (MILES, inches::to_miles),
                    (MILLIMETRES, inches::to_millimetres),
                    (NAUTICAL_MILES, inches::to_nautical_miles),
                    (YARDS, inches::to_yards),
                ]),
            ),
            (
                KILOMETRES,
                HashMap::from_iter([
                    (
                        CENTIMETRES,
                        kilometres::to_centimetres as ConversionFunction,
                    ),
                    (FEET, kilometres::to_feet),
                    (INCHES, kilometres::to_inches),
                    (KILOMETRES, identity),
                    (METRES, kilometres::to_metres),
                    (MILES, kilometres::to_miles),
                    (MILLIMETRES, kilometres::to_millimetres),
                    (NAUTICAL_MILES, kilometres::to_nautical_miles),
                    (YARDS, kilometres::to_yards),
                ]),
            ),
            (
                METRES,
                HashMap::from_iter([
                    (CENTIMETRES, metres::to_centimetres as ConversionFunction),
                    (FEET, metres::to_feet),
                    (INCHES, metres::to_inches),
                    (KILOMETRES, metres::to_kilometres),
                    (METRES, identity),
                    (MILES, metres::to_miles),
                    (MILLIMETRES, metres::to_millimetres),
                    (NAUTICAL_MILES, metres::to_nautical_miles),
                    (YARDS, metres::to_yards),
                ]),
            ),
            (
                MILES,
                HashMap::from_iter([
                    (CENTIMETRES, miles::to_centimetres as ConversionFunction),
                    (FEET, miles::to_feet),
                    (INCHES, miles::to_inches),
                    (KILOMETRES, miles::to_kilometres),
                    (METRES, miles::to_metres),
                    (MILES, identity),
                    (MILLIMETRES, miles::to_millimetres),
                    (NAUTICAL_MILES, miles::to_nautical_miles),
                    (YARDS, miles::to_yards),
                ]),
            ),
            (
                MILLIMETRES,
                HashMap::from_iter([
                    (
                        CENTIMETRES,
                        millimetres::to_centimetres as ConversionFunction,
                    ),
                    (FEET, millimetres::to_feet),
                    (INCHES, millimetres::to_inches),
                    (KILOMETRES, millimetres::to_kilometres),
                    (METRES, millimetres::to_metres),
                    (MILES, millimetres::to_miles),
                    (MILLIMETRES, identity),
                    (NAUTICAL_MILES, millimetres::to_nautical_miles),
                    (YARDS, millimetres::to_yards),
                ]),
            ),
            (
                NAUTICAL_MILES,
                HashMap::from_iter([
                    (
                        CENTIMETRES,
                        nautical_miles::to_centimetres as ConversionFunction,
                    ),
                    (FEET, nautical_miles::to_feet),
                    (INCHES, nautical_miles::to_inches),
                    (KILOMETRES, nautical_miles::to_kilometres),
                    (METRES, nautical_miles::to_metres),
                    (MILES, nautical_miles::to_miles),
                    (MILLIMETRES, nautical_miles::to_millimetres),
                    (NAUTICAL_MILES, identity),
                    (YARDS, nautical_miles::to_yards),
                ]),
            ),
            (
                YARDS,
                HashMap::from_iter([
                    (CENTIMETRES, yards::to_centimetres as ConversionFunction),
                    (FEET, yards::to_feet),
                    (INCHES, yards::to_inches),
                    (KILOMETRES, yards::to_kilometres),
                    (METRES, yards::to_metres),
                    (MILES, yards::to_miles),
                    (MILLIMETRES, yards::to_millimetres),
                    (NAUTICAL_MILES, yards::to_nautical_miles),
                    (YARDS, identity),
                ]),
            ),
        ])
    }
}
