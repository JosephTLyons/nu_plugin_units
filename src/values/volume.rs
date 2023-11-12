use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::volume::*;

pub struct Volume;

impl Values for Volume {
    fn name() -> &'static str {
        "volume"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "cubic-feet",
                HashMap::from_iter([
                    ("cubic-feet", identity as ConversionFunction),
                    ("cubic-inches", cubic_feet::to_cubic_inches),
                    ("cubic-metres", cubic_feet::to_cubic_metres),
                    ("fluid-ounces", cubic_feet::to_fluid_ounces),
                    ("gallons", cubic_feet::to_gallons),
                    ("kilolitres", cubic_feet::to_kilolitres),
                    ("litres", cubic_feet::to_litres),
                    ("millilitres", cubic_feet::to_millilitres),
                    ("oil-barrels", cubic_feet::to_oil_barrels),
                    ("pints", cubic_feet::to_pints),
                    ("quarts", cubic_feet::to_quarts),
                    ("tablespoons", cubic_feet::to_tablespoons),
                    ("teaspoons", cubic_feet::to_teaspoons),
                    ("us-cups", cubic_feet::to_u_s_cups),
                    ("us-fluid-ounces", cubic_feet::to_u_s_fluid_ounces),
                    ("us-gallons", cubic_feet::to_u_s_gallons),
                    ("us-pints", cubic_feet::to_u_s_pints),
                    ("us-quarts", cubic_feet::to_u_s_quarts),
                    ("us-tablespoons", cubic_feet::to_u_s_tablespoons),
                    ("us-teaspoons", cubic_feet::to_u_s_teaspoons),
                ]),
            ),
            (
                "cubic-inches",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        cubic_inches::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", identity),
                    ("cubic-metres", cubic_inches::to_cubic_metres),
                    ("fluid-ounces", cubic_inches::to_fluid_ounces),
                    ("gallons", cubic_inches::to_gallons),
                    ("kilolitres", cubic_inches::to_kilolitres),
                    ("litres", cubic_inches::to_litres),
                    ("millilitres", cubic_inches::to_millilitres),
                    ("oil-barrels", cubic_inches::to_oil_barrels),
                    ("pints", cubic_inches::to_pints),
                    ("quarts", cubic_inches::to_quarts),
                    ("tablespoons", cubic_inches::to_tablespoons),
                    ("teaspoons", cubic_inches::to_teaspoons),
                    ("us-cups", cubic_inches::to_u_s_cups),
                    ("us-fluid-ounces", cubic_inches::to_u_s_fluid_ounces),
                    ("us-gallons", cubic_inches::to_u_s_gallons),
                    ("us-pints", cubic_inches::to_u_s_pints),
                    ("us-quarts", cubic_inches::to_u_s_quarts),
                    ("us-tablespoons", cubic_inches::to_u_s_tablespoons),
                    ("us-teaspoons", cubic_inches::to_u_s_teaspoons),
                ]),
            ),
            (
                "cubic-metres",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        cubic_metres::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", cubic_metres::to_cubic_inches),
                    ("cubic-metres", identity),
                    ("fluid-ounces", cubic_metres::to_fluid_ounces),
                    ("gallons", cubic_metres::to_gallons),
                    ("kilolitres", cubic_metres::to_kilolitres),
                    ("litres", cubic_metres::to_litres),
                    ("millilitres", cubic_metres::to_millilitres),
                    ("oil-barrels", cubic_metres::to_oil_barrels),
                    ("pints", cubic_metres::to_pints),
                    ("quarts", cubic_metres::to_quarts),
                    ("tablespoons", cubic_metres::to_tablespoons),
                    ("teaspoons", cubic_metres::to_teaspoons),
                    ("us-cups", cubic_metres::to_u_s_cups),
                    ("us-fluid-ounces", cubic_metres::to_u_s_fluid_ounces),
                    ("us-gallons", cubic_metres::to_u_s_gallons),
                    ("us-pints", cubic_metres::to_u_s_pints),
                    ("us-quarts", cubic_metres::to_u_s_quarts),
                    ("us-tablespoons", cubic_metres::to_u_s_tablespoons),
                    ("us-teaspoons", cubic_metres::to_u_s_teaspoons),
                ]),
            ),
            (
                "fluid-ounces",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        fluid_ounces::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", fluid_ounces::to_cubic_inches),
                    ("cubic-metres", fluid_ounces::to_cubic_metres),
                    ("fluid-ounces", identity),
                    ("gallons", fluid_ounces::to_gallons),
                    ("kilolitres", fluid_ounces::to_kilolitres),
                    ("litres", fluid_ounces::to_litres),
                    ("millilitres", fluid_ounces::to_millilitres),
                    ("oil-barrels", fluid_ounces::to_oil_barrels),
                    ("pints", fluid_ounces::to_pints),
                    ("quarts", fluid_ounces::to_quarts),
                    ("tablespoons", fluid_ounces::to_tablespoons),
                    ("teaspoons", fluid_ounces::to_teaspoons),
                    ("us-cups", fluid_ounces::to_u_s_cups),
                    ("us-fluid-ounces", fluid_ounces::to_u_s_fluid_ounces),
                    ("us-gallons", fluid_ounces::to_u_s_gallons),
                    ("us-pints", fluid_ounces::to_u_s_pints),
                    ("us-quarts", fluid_ounces::to_u_s_quarts),
                    ("us-tablespoons", fluid_ounces::to_u_s_tablespoons),
                    ("us-teaspoons", fluid_ounces::to_u_s_teaspoons),
                ]),
            ),
            (
                "gallons",
                HashMap::from_iter([
                    ("cubic-feet", gallons::to_cubic_feet as ConversionFunction),
                    ("cubic-inches", gallons::to_cubic_inches),
                    ("cubic-metres", gallons::to_cubic_metres),
                    ("fluid-ounces", gallons::to_fluid_ounces),
                    ("gallons", identity),
                    ("kilolitres", gallons::to_kilolitres),
                    ("litres", gallons::to_litres),
                    ("millilitres", gallons::to_millilitres),
                    ("oil-barrels", gallons::to_oil_barrels),
                    ("pints", gallons::to_pints),
                    ("quarts", gallons::to_quarts),
                    ("tablespoons", gallons::to_tablespoons),
                    ("teaspoons", gallons::to_teaspoons),
                    ("us-cups", gallons::to_u_s_cups),
                    ("us-fluid-ounces", gallons::to_u_s_fluid_ounces),
                    ("us-gallons", gallons::to_u_s_gallons),
                    ("us-pints", gallons::to_u_s_pints),
                    ("us-quarts", gallons::to_u_s_quarts),
                    ("us-tablespoons", gallons::to_u_s_tablespoons),
                    ("us-teaspoons", gallons::to_u_s_teaspoons),
                ]),
            ),
            (
                "kilolitres",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        kilolitres::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", kilolitres::to_cubic_inches),
                    ("cubic-metres", kilolitres::to_cubic_metres),
                    ("fluid-ounces", kilolitres::to_fluid_ounces),
                    ("gallons", kilolitres::to_gallons),
                    ("kilolitres", identity),
                    ("litres", kilolitres::to_litres),
                    ("millilitres", kilolitres::to_millilitres),
                    ("oil-barrels", kilolitres::to_oil_barrels),
                    ("pints", kilolitres::to_pints),
                    ("quarts", kilolitres::to_quarts),
                    ("tablespoons", kilolitres::to_tablespoons),
                    ("teaspoons", kilolitres::to_teaspoons),
                    ("us-cups", kilolitres::to_u_s_cups),
                    ("us-fluid-ounces", kilolitres::to_u_s_fluid_ounces),
                    ("us-gallons", kilolitres::to_u_s_gallons),
                    ("us-pints", kilolitres::to_u_s_pints),
                    ("us-quarts", kilolitres::to_u_s_quarts),
                    ("us-tablespoons", kilolitres::to_u_s_tablespoons),
                    ("us-teaspoons", kilolitres::to_u_s_teaspoons),
                ]),
            ),
            (
                "litres",
                HashMap::from_iter([
                    ("cubic-feet", litres::to_cubic_feet as ConversionFunction),
                    ("cubic-inches", litres::to_cubic_inches),
                    ("cubic-metres", litres::to_cubic_metres),
                    ("fluid-ounces", litres::to_fluid_ounces),
                    ("gallons", litres::to_gallons),
                    ("kilolitres", litres::to_kilolitres),
                    ("litres", identity),
                    ("millilitres", litres::to_millilitres),
                    ("oil-barrels", litres::to_oil_barrels),
                    ("pints", litres::to_pints),
                    ("quarts", litres::to_quarts),
                    ("tablespoons", litres::to_tablespoons),
                    ("teaspoons", litres::to_teaspoons),
                    ("us-cups", litres::to_u_s_cups),
                    ("us-fluid-ounces", litres::to_u_s_fluid_ounces),
                    ("us-gallons", litres::to_u_s_gallons),
                    ("us-pints", litres::to_u_s_pints),
                    ("us-quarts", litres::to_u_s_quarts),
                    ("us-tablespoons", litres::to_u_s_tablespoons),
                    ("us-teaspoons", litres::to_u_s_teaspoons),
                ]),
            ),
            (
                "millilitres",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        millilitres::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", millilitres::to_cubic_inches),
                    ("cubic-metres", millilitres::to_cubic_metres),
                    ("fluid-ounces", millilitres::to_fluid_ounces),
                    ("gallons", millilitres::to_gallons),
                    ("kilolitres", millilitres::to_kilolitres),
                    ("litres", millilitres::to_litres),
                    ("millilitres", identity),
                    ("oil-barrels", millilitres::to_oil_barrels),
                    ("pints", millilitres::to_pints),
                    ("quarts", millilitres::to_quarts),
                    ("tablespoons", millilitres::to_tablespoons),
                    ("teaspoons", millilitres::to_teaspoons),
                    ("us-cups", millilitres::to_u_s_cups),
                    ("us-fluid-ounces", millilitres::to_u_s_fluid_ounces),
                    ("us-gallons", millilitres::to_u_s_gallons),
                    ("us-pints", millilitres::to_u_s_pints),
                    ("us-quarts", millilitres::to_u_s_quarts),
                    ("us-tablespoons", millilitres::to_u_s_tablespoons),
                    ("us-teaspoons", millilitres::to_u_s_teaspoons),
                ]),
            ),
            (
                "oil-barrels",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        oil_barrels::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", oil_barrels::to_cubic_inches),
                    ("cubic-metres", oil_barrels::to_cubic_metres),
                    ("fluid-ounces", oil_barrels::to_fluid_ounces),
                    ("gallons", oil_barrels::to_gallons),
                    ("kilolitres", oil_barrels::to_kilolitres),
                    ("litres", oil_barrels::to_litres),
                    ("millilitres", oil_barrels::to_millilitres),
                    ("oil-barrels", identity),
                    ("pints", oil_barrels::to_pints),
                    ("quarts", oil_barrels::to_quarts),
                    ("tablespoons", oil_barrels::to_tablespoons),
                    ("teaspoons", oil_barrels::to_teaspoons),
                    ("us-cups", oil_barrels::to_u_s_cups),
                    ("us-fluid-ounces", oil_barrels::to_u_s_fluid_ounces),
                    ("us-gallons", oil_barrels::to_u_s_gallons),
                    ("us-pints", oil_barrels::to_u_s_pints),
                    ("us-quarts", oil_barrels::to_u_s_quarts),
                    ("us-tablespoons", oil_barrels::to_u_s_tablespoons),
                    ("us-teaspoons", oil_barrels::to_u_s_teaspoons),
                ]),
            ),
            (
                "pints",
                HashMap::from_iter([
                    ("cubic-feet", pints::to_cubic_feet as ConversionFunction),
                    ("cubic-inches", pints::to_cubic_inches),
                    ("cubic-metres", pints::to_cubic_metres),
                    ("fluid-ounces", pints::to_fluid_ounces),
                    ("gallons", pints::to_gallons),
                    ("kilolitres", pints::to_kilolitres),
                    ("litres", pints::to_litres),
                    ("millilitres", pints::to_millilitres),
                    ("oil-barrels", pints::to_oil_barrels),
                    ("pints", identity),
                    ("quarts", pints::to_quarts),
                    ("tablespoons", pints::to_tablespoons),
                    ("teaspoons", pints::to_teaspoons),
                    ("us-cups", pints::to_u_s_cups),
                    ("us-fluid-ounces", pints::to_u_s_fluid_ounces),
                    ("us-gallons", pints::to_u_s_gallons),
                    ("us-pints", pints::to_u_s_pints),
                    ("us-quarts", pints::to_u_s_quarts),
                    ("us-tablespoons", pints::to_u_s_tablespoons),
                    ("us-teaspoons", pints::to_u_s_teaspoons),
                ]),
            ),
            (
                "quarts",
                HashMap::from_iter([
                    ("cubic-feet", quarts::to_cubic_feet as ConversionFunction),
                    ("cubic-inches", quarts::to_cubic_inches),
                    ("cubic-metres", quarts::to_cubic_metres),
                    ("fluid-ounces", quarts::to_fluid_ounces),
                    ("gallons", quarts::to_gallons),
                    ("kilolitres", quarts::to_kilolitres),
                    ("litres", quarts::to_litres),
                    ("millilitres", quarts::to_millilitres),
                    ("oil-barrels", quarts::to_oil_barrels),
                    ("pints", quarts::to_pints),
                    ("quarts", identity),
                    ("tablespoons", quarts::to_tablespoons),
                    ("teaspoons", quarts::to_teaspoons),
                    ("us-cups", quarts::to_u_s_cups),
                    ("us-fluid-ounces", quarts::to_u_s_fluid_ounces),
                    ("us-gallons", quarts::to_u_s_gallons),
                    ("us-pints", quarts::to_u_s_pints),
                    ("us-quarts", quarts::to_u_s_quarts),
                    ("us-tablespoons", quarts::to_u_s_tablespoons),
                    ("us-teaspoons", quarts::to_u_s_teaspoons),
                ]),
            ),
            (
                "tablespoons",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        tablespoons::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", tablespoons::to_cubic_inches),
                    ("cubic-metres", tablespoons::to_cubic_metres),
                    ("fluid-ounces", tablespoons::to_fluid_ounces),
                    ("gallons", tablespoons::to_gallons),
                    ("kilolitres", tablespoons::to_kilolitres),
                    ("litres", tablespoons::to_litres),
                    ("millilitres", tablespoons::to_millilitres),
                    ("oil-barrels", tablespoons::to_oil_barrels),
                    ("pints", tablespoons::to_pints),
                    ("quarts", tablespoons::to_quarts),
                    ("tablespoons", identity),
                    ("teaspoons", tablespoons::to_teaspoons),
                    ("us-cups", tablespoons::to_u_s_cups),
                    ("us-fluid-ounces", tablespoons::to_u_s_fluid_ounces),
                    ("us-gallons", tablespoons::to_u_s_gallons),
                    ("us-pints", tablespoons::to_u_s_pints),
                    ("us-quarts", tablespoons::to_u_s_quarts),
                    ("us-tablespoons", tablespoons::to_u_s_tablespoons),
                    ("us-teaspoons", tablespoons::to_u_s_teaspoons),
                ]),
            ),
            (
                "teaspoons",
                HashMap::from_iter([
                    ("cubic-feet", teaspoons::to_cubic_feet as ConversionFunction),
                    ("cubic-inches", teaspoons::to_cubic_inches),
                    ("cubic-metres", teaspoons::to_cubic_metres),
                    ("fluid-ounces", teaspoons::to_fluid_ounces),
                    ("gallons", teaspoons::to_gallons),
                    ("kilolitres", teaspoons::to_kilolitres),
                    ("litres", teaspoons::to_litres),
                    ("millilitres", teaspoons::to_millilitres),
                    ("oil-barrels", teaspoons::to_oil_barrels),
                    ("pints", teaspoons::to_pints),
                    ("quarts", teaspoons::to_quarts),
                    ("tablespoons", teaspoons::to_tablespoons),
                    ("teaspoons", identity),
                    ("us-cups", teaspoons::to_u_s_cups),
                    ("us-fluid-ounces", teaspoons::to_u_s_fluid_ounces),
                    ("us-gallons", teaspoons::to_u_s_gallons),
                    ("us-pints", teaspoons::to_u_s_pints),
                    ("us-quarts", teaspoons::to_u_s_quarts),
                    ("us-tablespoons", teaspoons::to_u_s_tablespoons),
                    ("us-teaspoons", teaspoons::to_u_s_teaspoons),
                ]),
            ),
            (
                "us-cups",
                HashMap::from_iter([
                    ("cubic-feet", u_s_cups::to_cubic_feet as ConversionFunction),
                    ("cubic-inches", u_s_cups::to_cubic_inches),
                    ("cubic-metres", u_s_cups::to_cubic_metres),
                    ("fluid-ounces", u_s_cups::to_fluid_ounces),
                    ("gallons", u_s_cups::to_gallons),
                    ("kilolitres", u_s_cups::to_kilolitres),
                    ("litres", u_s_cups::to_litres),
                    ("millilitres", u_s_cups::to_millilitres),
                    ("oil-barrels", u_s_cups::to_oil_barrels),
                    ("pints", u_s_cups::to_pints),
                    ("quarts", u_s_cups::to_quarts),
                    ("tablespoons", u_s_cups::to_tablespoons),
                    ("teaspoons", u_s_cups::to_teaspoons),
                    ("us-cups", identity),
                    ("us-fluid-ounces", u_s_cups::to_u_s_fluid_ounces),
                    ("us-gallons", u_s_cups::to_u_s_gallons),
                    ("us-pints", u_s_cups::to_u_s_pints),
                    ("us-quarts", u_s_cups::to_u_s_quarts),
                    ("us-tablespoons", u_s_cups::to_u_s_tablespoons),
                    ("us-teaspoons", u_s_cups::to_u_s_teaspoons),
                ]),
            ),
            (
                "us-fluid-ounces",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        u_s_fluid_ounces::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", u_s_fluid_ounces::to_cubic_inches),
                    ("cubic-metres", u_s_fluid_ounces::to_cubic_metres),
                    ("fluid-ounces", u_s_fluid_ounces::to_fluid_ounces),
                    ("gallons", u_s_fluid_ounces::to_gallons),
                    ("kilolitres", u_s_fluid_ounces::to_kilolitres),
                    ("litres", u_s_fluid_ounces::to_litres),
                    ("millilitres", u_s_fluid_ounces::to_millilitres),
                    ("oil-barrels", u_s_fluid_ounces::to_oil_barrels),
                    ("pints", u_s_fluid_ounces::to_pints),
                    ("quarts", u_s_fluid_ounces::to_quarts),
                    ("tablespoons", u_s_fluid_ounces::to_tablespoons),
                    ("teaspoons", u_s_fluid_ounces::to_teaspoons),
                    ("us-cups", u_s_fluid_ounces::to_u_s_cups),
                    ("us-fluid-ounces", identity),
                    ("us-gallons", u_s_fluid_ounces::to_u_s_gallons),
                    ("us-pints", u_s_fluid_ounces::to_u_s_pints),
                    ("us-quarts", u_s_fluid_ounces::to_u_s_quarts),
                    ("us-tablespoons", u_s_fluid_ounces::to_u_s_tablespoons),
                    ("us-teaspoons", u_s_fluid_ounces::to_u_s_teaspoons),
                ]),
            ),
            (
                "us-gallons",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        u_s_gallons::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", u_s_gallons::to_cubic_inches),
                    ("cubic-metres", u_s_gallons::to_cubic_metres),
                    ("fluid-ounces", u_s_gallons::to_fluid_ounces),
                    ("gallons", u_s_gallons::to_gallons),
                    ("kilolitres", u_s_gallons::to_kilolitres),
                    ("litres", u_s_gallons::to_litres),
                    ("millilitres", u_s_gallons::to_millilitres),
                    ("oil-barrels", u_s_gallons::to_oil_barrels),
                    ("pints", u_s_gallons::to_pints),
                    ("quarts", u_s_gallons::to_quarts),
                    ("tablespoons", u_s_gallons::to_tablespoons),
                    ("teaspoons", u_s_gallons::to_teaspoons),
                    ("us-cups", u_s_gallons::to_u_s_cups),
                    ("us-fluid-ounces", u_s_gallons::to_u_s_fluid_ounces),
                    ("us-gallons", identity),
                    ("us-pints", u_s_gallons::to_u_s_pints),
                    ("us-quarts", u_s_gallons::to_u_s_quarts),
                    ("us-tablespoons", u_s_gallons::to_u_s_tablespoons),
                    ("us-teaspoons", u_s_gallons::to_u_s_teaspoons),
                ]),
            ),
            (
                "us-pints",
                HashMap::from_iter([
                    ("cubic-feet", u_s_pints::to_cubic_feet as ConversionFunction),
                    ("cubic-inches", u_s_pints::to_cubic_inches),
                    ("cubic-metres", u_s_pints::to_cubic_metres),
                    ("fluid-ounces", u_s_pints::to_fluid_ounces),
                    ("gallons", u_s_pints::to_gallons),
                    ("kilolitres", u_s_pints::to_kilolitres),
                    ("litres", u_s_pints::to_litres),
                    ("millilitres", u_s_pints::to_millilitres),
                    ("oil-barrels", u_s_pints::to_oil_barrels),
                    ("pints", u_s_pints::to_pints),
                    ("quarts", u_s_pints::to_quarts),
                    ("tablespoons", u_s_pints::to_tablespoons),
                    ("teaspoons", u_s_pints::to_teaspoons),
                    ("us-cups", u_s_pints::to_u_s_cups),
                    ("us-fluid-ounces", u_s_pints::to_u_s_fluid_ounces),
                    ("us-gallons", u_s_pints::to_u_s_gallons),
                    ("us-pints", identity),
                    ("us-quarts", u_s_pints::to_u_s_quarts),
                    ("us-tablespoons", u_s_pints::to_u_s_tablespoons),
                    ("us-teaspoons", u_s_pints::to_u_s_teaspoons),
                ]),
            ),
            (
                "us-quarts",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        u_s_quarts::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", u_s_quarts::to_cubic_inches),
                    ("cubic-metres", u_s_quarts::to_cubic_metres),
                    ("fluid-ounces", u_s_quarts::to_fluid_ounces),
                    ("gallons", u_s_quarts::to_gallons),
                    ("kilolitres", u_s_quarts::to_kilolitres),
                    ("litres", u_s_quarts::to_litres),
                    ("millilitres", u_s_quarts::to_millilitres),
                    ("oil-barrels", u_s_quarts::to_oil_barrels),
                    ("pints", u_s_quarts::to_pints),
                    ("quarts", u_s_quarts::to_quarts),
                    ("tablespoons", u_s_quarts::to_tablespoons),
                    ("teaspoons", u_s_quarts::to_teaspoons),
                    ("us-cups", u_s_quarts::to_u_s_cups),
                    ("us-fluid-ounces", u_s_quarts::to_u_s_fluid_ounces),
                    ("us-gallons", u_s_quarts::to_u_s_gallons),
                    ("us-pints", u_s_quarts::to_u_s_pints),
                    ("us-quarts", identity),
                    ("us-tablespoons", u_s_quarts::to_u_s_tablespoons),
                    ("us-teaspoons", u_s_quarts::to_u_s_teaspoons),
                ]),
            ),
            (
                "us-tablespoons",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        u_s_tablespoons::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", u_s_tablespoons::to_cubic_inches),
                    ("cubic-metres", u_s_tablespoons::to_cubic_metres),
                    ("fluid-ounces", u_s_tablespoons::to_fluid_ounces),
                    ("gallons", u_s_tablespoons::to_gallons),
                    ("kilolitres", u_s_tablespoons::to_kilolitres),
                    ("litres", u_s_tablespoons::to_litres),
                    ("millilitres", u_s_tablespoons::to_millilitres),
                    ("oil-barrels", u_s_tablespoons::to_oil_barrels),
                    ("pints", u_s_tablespoons::to_pints),
                    ("quarts", u_s_tablespoons::to_quarts),
                    ("tablespoons", u_s_tablespoons::to_tablespoons),
                    ("teaspoons", u_s_tablespoons::to_teaspoons),
                    ("us-cups", u_s_tablespoons::to_u_s_cups),
                    ("us-fluid-ounces", u_s_tablespoons::to_u_s_fluid_ounces),
                    ("us-gallons", u_s_tablespoons::to_u_s_gallons),
                    ("us-pints", u_s_tablespoons::to_u_s_pints),
                    ("us-quarts", u_s_tablespoons::to_u_s_quarts),
                    ("us-tablespoons", identity),
                    ("us-teaspoons", u_s_tablespoons::to_u_s_teaspoons),
                ]),
            ),
            (
                "us-teaspoons",
                HashMap::from_iter([
                    (
                        "cubic-feet",
                        u_s_teaspoons::to_cubic_feet as ConversionFunction,
                    ),
                    ("cubic-inches", u_s_teaspoons::to_cubic_inches),
                    ("cubic-metres", u_s_teaspoons::to_cubic_metres),
                    ("fluid-ounces", u_s_teaspoons::to_fluid_ounces),
                    ("gallons", u_s_teaspoons::to_gallons),
                    ("kilolitres", u_s_teaspoons::to_kilolitres),
                    ("litres", u_s_teaspoons::to_litres),
                    ("millilitres", u_s_teaspoons::to_millilitres),
                    ("oil-barrels", u_s_teaspoons::to_oil_barrels),
                    ("pints", u_s_teaspoons::to_pints),
                    ("quarts", u_s_teaspoons::to_quarts),
                    ("tablespoons", u_s_teaspoons::to_tablespoons),
                    ("teaspoons", u_s_teaspoons::to_teaspoons),
                    ("us-cups", u_s_teaspoons::to_u_s_cups),
                    ("us-fluid-ounces", u_s_teaspoons::to_u_s_fluid_ounces),
                    ("us-gallons", u_s_teaspoons::to_u_s_gallons),
                    ("us-pints", u_s_teaspoons::to_u_s_pints),
                    ("us-quarts", u_s_teaspoons::to_u_s_quarts),
                    ("us-tablespoons", u_s_teaspoons::to_u_s_tablespoons),
                    ("us-teaspoons", identity),
                ]),
            ),
        ])
    }
}
