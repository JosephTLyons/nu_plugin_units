use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::fuel_economy::*;

pub struct FuelEconomy;

impl Values for FuelEconomy {
    fn name() -> &'static str {
        "fuel-economy"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "kilometre_per_litre",
                HashMap::from_iter([
                    ("kilometre_per_litre", identity as ConversionFunction),
                    (
                        "litres-per-100-kilometres",
                        kilometre_per_litre::to_litres_per100_kilometres,
                    ),
                    ("miles-per-gallon", kilometre_per_litre::to_miles_per_gallon),
                    (
                        "u-s-miles-per-gallon",
                        kilometre_per_litre::to_u_s_miles_per_gallon,
                    ),
                ]),
            ),
            (
                "litres-per-100-kilometres",
                HashMap::from_iter([
                    (
                        "kilometre_per_litre",
                        litres_per100_kilometres::to_kilometre_per_litre as ConversionFunction,
                    ),
                    ("litres-per-100-kilometres", identity),
                    (
                        "miles-per-gallon",
                        litres_per100_kilometres::to_miles_per_gallon,
                    ),
                    (
                        "u-s-miles-per-gallon",
                        litres_per100_kilometres::to_u_s_miles_per_gallon,
                    ),
                ]),
            ),
            (
                "miles-per-gallon",
                HashMap::from_iter([
                    (
                        "kilometre_per_litre",
                        miles_per_gallon::to_kilometre_per_litre as ConversionFunction,
                    ),
                    (
                        "litres-per-100-kilometres",
                        miles_per_gallon::to_litres_per100_kilometres,
                    ),
                    ("miles-per-gallon", identity),
                    (
                        "u-s-miles-per-gallon",
                        miles_per_gallon::to_u_s_miles_per_gallon,
                    ),
                ]),
            ),
            (
                "u-s-miles-per-gallon",
                HashMap::from_iter([
                    (
                        "kilometre_per_litre",
                        u_s_miles_per_gallon::to_kilometre_per_litre as ConversionFunction,
                    ),
                    (
                        "litres-per-100-kilometres",
                        u_s_miles_per_gallon::to_litres_per100_kilometres,
                    ),
                    (
                        "miles-per-gallon",
                        u_s_miles_per_gallon::to_miles_per_gallon,
                    ),
                    ("u-s-miles-per-gallon", identity),
                ]),
            ),
        ])
    }
}
