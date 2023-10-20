use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::volume::*;

pub struct Volume;

impl Values for Volume {
    fn name() -> &'static str {
        "volume"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "cubic-feet",
                cubic_feet::to_u_s_cups,
                u_s_cups::to_cubic_feet,
            ),
            conversion(
                "cubic-inches",
                cubic_inches::to_u_s_cups,
                u_s_cups::to_cubic_inches,
            ),
            conversion(
                "cubic-metres",
                cubic_metres::to_u_s_cups,
                u_s_cups::to_cubic_metres,
            ),
            conversion(
                "fluid-ounces",
                fluid_ounces::to_u_s_cups,
                u_s_cups::to_fluid_ounces,
            ),
            conversion("gallons", gallons::to_u_s_cups, u_s_cups::to_gallons),
            conversion(
                "kilolitres",
                kilolitres::to_u_s_cups,
                u_s_cups::to_kilolitres,
            ),
            conversion("litres", litres::to_u_s_cups, u_s_cups::to_litres),
            conversion(
                "millilitres",
                millilitres::to_u_s_cups,
                u_s_cups::to_millilitres,
            ),
            conversion(
                "oil-barrels",
                oil_barrels::to_u_s_cups,
                u_s_cups::to_oil_barrels,
            ),
            conversion("pints", pints::to_u_s_cups, u_s_cups::to_pints),
            conversion("quarts", quarts::to_u_s_cups, u_s_cups::to_quarts),
            conversion(
                "tablespoons",
                tablespoons::to_u_s_cups,
                u_s_cups::to_tablespoons,
            ),
            conversion("teaspoons", teaspoons::to_u_s_cups, u_s_cups::to_teaspoons),
            conversion("teaspoons", teaspoons::to_u_s_cups, u_s_cups::to_teaspoons),
            conversion("us-cups", identity, identity),
            conversion(
                "us-fluid-ounces",
                u_s_fluid_ounces::to_u_s_cups,
                u_s_cups::to_u_s_fluid_ounces,
            ),
            conversion(
                "us-gallons",
                u_s_gallons::to_u_s_cups,
                u_s_cups::to_u_s_gallons,
            ),
            conversion("us-pints", u_s_pints::to_u_s_cups, u_s_cups::to_u_s_pints),
            conversion(
                "us-quarts",
                u_s_quarts::to_u_s_cups,
                u_s_cups::to_u_s_quarts,
            ),
            conversion(
                "us-tablespoons",
                u_s_tablespoons::to_u_s_cups,
                u_s_cups::to_u_s_tablespoons,
            ),
            conversion(
                "us-teaspoons",
                u_s_teaspoons::to_u_s_cups,
                u_s_cups::to_u_s_teaspoons,
            ),
        ])
    }
}
