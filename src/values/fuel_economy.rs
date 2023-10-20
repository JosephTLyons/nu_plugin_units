use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::fuel_economy::*;

pub struct FuelEconomy;

impl Values for FuelEconomy {
    fn name() -> &'static str {
        "fuel-economy"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "kilometre_per_litre",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "litres-per-100-kilometres",
                litres_per100_kilometres::to_kilometre_per_litre as BaseConversionFunction,
                kilometre_per_litre::to_litres_per100_kilometres as BaseConversionFunction,
            ),
            conversion(
                "miles-per-gallon",
                miles_per_gallon::to_kilometre_per_litre,
                kilometre_per_litre::to_miles_per_gallon,
            ),
            conversion(
                "u-s-miles-per-gallon",
                u_s_miles_per_gallon::to_kilometre_per_litre,
                kilometre_per_litre::to_u_s_miles_per_gallon,
            ),
        ])
    }
}
