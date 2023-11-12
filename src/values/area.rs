use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::area::*;

pub struct Area;

impl Values for Area {
    fn name() -> &'static str {
        "area"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "acres",
                HashMap::from_iter([
                    ("acres", identity as ConversionFunction),
                    ("hectares", acres::to_hectares),
                    ("square-feet", acres::to_square_feet),
                    ("square-inches", acres::to_square_inches),
                    ("square-kilometres", acres::to_square_kilometres),
                    ("square-metres", acres::to_square_metres),
                    ("square-miles", acres::to_square_miles),
                ]),
            ),
            (
                "hectares",
                HashMap::from_iter([
                    ("acres", hectares::to_acres as ConversionFunction),
                    ("hectares", identity),
                    ("square-feet", hectares::to_square_feet),
                    ("square-inches", hectares::to_square_inches),
                    ("square-kilometres", hectares::to_square_kilometres),
                    ("square-metres", hectares::to_square_metres),
                    ("square-miles", hectares::to_square_miles),
                ]),
            ),
            (
                "square-feet",
                HashMap::from_iter([
                    ("acres", square_feet::to_acres as ConversionFunction),
                    ("hectares", square_feet::to_hectares),
                    ("square-feet", identity),
                    ("square-inches", square_feet::to_square_inches),
                    ("square-kilometres", square_feet::to_square_kilometres),
                    ("square-metres", square_feet::to_square_metres),
                    ("square-miles", square_feet::to_square_miles),
                ]),
            ),
            (
                "square-inches",
                HashMap::from_iter([
                    ("acres", square_inches::to_acres as ConversionFunction),
                    ("hectares", square_inches::to_hectares),
                    ("square-feet", square_inches::to_square_feet),
                    ("square-inches", identity),
                    ("square-kilometres", square_inches::to_square_kilometres),
                    ("square-metres", square_inches::to_square_metres),
                    ("square-miles", square_inches::to_square_miles),
                ]),
            ),
            (
                "square-kilometres",
                HashMap::from_iter([
                    ("acres", square_kilometres::to_acres as ConversionFunction),
                    ("hectares", square_kilometres::to_hectares),
                    ("square-feet", square_kilometres::to_square_feet),
                    ("square-inches", square_kilometres::to_square_inches),
                    ("square-kilometres", identity),
                    ("square-metres", square_kilometres::to_square_metres),
                    ("square-miles", square_kilometres::to_square_miles),
                ]),
            ),
            (
                "square-metres",
                HashMap::from_iter([
                    ("acres", square_metres::to_acres as ConversionFunction),
                    ("hectares", square_metres::to_hectares),
                    ("square-feet", square_metres::to_square_feet),
                    ("square-inches", square_metres::to_square_inches),
                    ("square-kilometres", square_metres::to_square_kilometres),
                    ("square-metres", identity),
                    ("square-miles", square_metres::to_square_miles),
                ]),
            ),
            (
                "square-miles",
                HashMap::from_iter([
                    ("acres", square_miles::to_acres as ConversionFunction),
                    ("hectares", square_miles::to_hectares),
                    ("square-feet", square_miles::to_square_feet),
                    ("square-inches", square_miles::to_square_inches),
                    ("square-kilometres", square_miles::to_square_kilometres),
                    ("square-metres", square_miles::to_square_metres),
                    ("square-miles", identity),
                ]),
            ),
        ])
    }
}
