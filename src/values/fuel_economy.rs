use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::fuel_economy::*;

const KILOMETRES_PER_LITRE: &str = "kilometres-per-litre";
const LITRES_PER_100_KILOMETRES: &str = "litres-per-100-kilometres";
const MILES_PER_GALLON: &str = "miles-per-gallon";
const US_MILES_PER_GALLON: &str = "us-miles-per-gallon";

pub struct FuelEconomy;

impl Category for FuelEconomy {
    fn name() -> &'static str {
        "fuel-economy"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                KILOMETRES_PER_LITRE,
                HashMap::from_iter([
                    (KILOMETRES_PER_LITRE, identity as ConversionFunction),
                    (
                        LITRES_PER_100_KILOMETRES,
                        kilometre_per_litre::to_litres_per100_kilometres,
                    ),
                    (MILES_PER_GALLON, kilometre_per_litre::to_miles_per_gallon),
                    (
                        US_MILES_PER_GALLON,
                        kilometre_per_litre::to_u_s_miles_per_gallon,
                    ),
                ]),
            ),
            (
                LITRES_PER_100_KILOMETRES,
                HashMap::from_iter([
                    (
                        KILOMETRES_PER_LITRE,
                        litres_per100_kilometres::to_kilometre_per_litre as ConversionFunction,
                    ),
                    (LITRES_PER_100_KILOMETRES, identity),
                    (
                        MILES_PER_GALLON,
                        litres_per100_kilometres::to_miles_per_gallon,
                    ),
                    (
                        US_MILES_PER_GALLON,
                        litres_per100_kilometres::to_u_s_miles_per_gallon,
                    ),
                ]),
            ),
            (
                MILES_PER_GALLON,
                HashMap::from_iter([
                    (
                        KILOMETRES_PER_LITRE,
                        miles_per_gallon::to_kilometre_per_litre as ConversionFunction,
                    ),
                    (
                        LITRES_PER_100_KILOMETRES,
                        miles_per_gallon::to_litres_per100_kilometres,
                    ),
                    (MILES_PER_GALLON, identity),
                    (
                        US_MILES_PER_GALLON,
                        miles_per_gallon::to_u_s_miles_per_gallon,
                    ),
                ]),
            ),
            (
                US_MILES_PER_GALLON,
                HashMap::from_iter([
                    (
                        KILOMETRES_PER_LITRE,
                        u_s_miles_per_gallon::to_kilometre_per_litre as ConversionFunction,
                    ),
                    (
                        LITRES_PER_100_KILOMETRES,
                        u_s_miles_per_gallon::to_litres_per100_kilometres,
                    ),
                    (MILES_PER_GALLON, u_s_miles_per_gallon::to_miles_per_gallon),
                    (US_MILES_PER_GALLON, identity),
                ]),
            ),
        ])
    }
}
