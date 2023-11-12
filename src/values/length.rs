use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::length::*;

pub struct Length;

impl Values for Length {
    fn name() -> &'static str {
        "length"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "centimetres",
                HashMap::from_iter([
                    ("centimetres", identity as ConversionFunction),
                    ("feet", centimetres::to_feet),
                    ("inches", centimetres::to_inches),
                    ("kilometres", centimetres::to_kilometres),
                    ("metres", centimetres::to_metres),
                    ("miles", centimetres::to_miles),
                    ("millimetres", centimetres::to_millimetres),
                    ("nautical-miles", centimetres::to_nautical_miles),
                    ("yards", centimetres::to_yards),
                ]),
            ),
            (
                "feet",
                HashMap::from_iter([
                    ("centimetres", feet::to_centimetres as ConversionFunction),
                    ("feet", identity),
                    ("inches", feet::to_inches),
                    ("kilometres", feet::to_kilometres),
                    ("metres", feet::to_metres),
                    ("miles", feet::to_miles),
                    ("millimetres", feet::to_millimetres),
                    ("nautical-miles", feet::to_nautical_miles),
                    ("yards", feet::to_yards),
                ]),
            ),
            (
                "inches",
                HashMap::from_iter([
                    ("centimetres", inches::to_centimetres as ConversionFunction),
                    ("feet", inches::to_feet),
                    ("inches", identity),
                    ("kilometres", inches::to_kilometres),
                    ("metres", inches::to_metres),
                    ("miles", inches::to_miles),
                    ("millimetres", inches::to_millimetres),
                    ("nautical-miles", inches::to_nautical_miles),
                    ("yards", inches::to_yards),
                ]),
            ),
            (
                "kilometres",
                HashMap::from_iter([
                    (
                        "centimetres",
                        kilometres::to_centimetres as ConversionFunction,
                    ),
                    ("feet", kilometres::to_feet),
                    ("inches", kilometres::to_inches),
                    ("kilometres", identity),
                    ("metres", kilometres::to_metres),
                    ("miles", kilometres::to_miles),
                    ("millimetres", kilometres::to_millimetres),
                    ("nautical-miles", kilometres::to_nautical_miles),
                    ("yards", kilometres::to_yards),
                ]),
            ),
            (
                "metres",
                HashMap::from_iter([
                    ("centimetres", metres::to_centimetres as ConversionFunction),
                    ("feet", metres::to_feet),
                    ("inches", metres::to_inches),
                    ("kilometres", metres::to_kilometres),
                    ("metres", identity),
                    ("miles", metres::to_miles),
                    ("millimetres", metres::to_millimetres),
                    ("nautical-miles", metres::to_nautical_miles),
                    ("yards", metres::to_yards),
                ]),
            ),
            (
                "miles",
                HashMap::from_iter([
                    ("centimetres", miles::to_centimetres as ConversionFunction),
                    ("feet", miles::to_feet),
                    ("inches", miles::to_inches),
                    ("kilometres", miles::to_kilometres),
                    ("metres", miles::to_metres),
                    ("miles", identity),
                    ("millimetres", miles::to_millimetres),
                    ("nautical-miles", miles::to_nautical_miles),
                    ("yards", miles::to_yards),
                ]),
            ),
            (
                "millimetres",
                HashMap::from_iter([
                    (
                        "centimetres",
                        millimetres::to_centimetres as ConversionFunction,
                    ),
                    ("feet", millimetres::to_feet),
                    ("inches", millimetres::to_inches),
                    ("kilometres", millimetres::to_kilometres),
                    ("metres", millimetres::to_metres),
                    ("miles", millimetres::to_miles),
                    ("millimetres", identity),
                    ("nautical-miles", millimetres::to_nautical_miles),
                    ("yards", millimetres::to_yards),
                ]),
            ),
            (
                "nautical-miles",
                HashMap::from_iter([
                    (
                        "centimetres",
                        nautical_miles::to_centimetres as ConversionFunction,
                    ),
                    ("feet", nautical_miles::to_feet),
                    ("inches", nautical_miles::to_inches),
                    ("kilometres", nautical_miles::to_kilometres),
                    ("metres", nautical_miles::to_metres),
                    ("miles", nautical_miles::to_miles),
                    ("millimetres", nautical_miles::to_millimetres),
                    ("nautical-miles", identity),
                    ("yards", nautical_miles::to_yards),
                ]),
            ),
            (
                "yards",
                HashMap::from_iter([
                    ("centimetres", yards::to_centimetres as ConversionFunction),
                    ("feet", yards::to_feet),
                    ("inches", yards::to_inches),
                    ("kilometres", yards::to_kilometres),
                    ("metres", yards::to_metres),
                    ("miles", yards::to_miles),
                    ("millimetres", yards::to_millimetres),
                    ("nautical-miles", yards::to_nautical_miles),
                    ("yards", identity),
                ]),
            ),
        ])
    }
}
