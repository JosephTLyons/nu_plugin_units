use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::mass::*;

const CARATS: &str = "carats";
const GRAMS: &str = "grams";
const KILOGRAMS: &str = "kilograms";
const MILLIGRAMS: &str = "milligrams";
const OUNCES: &str = "ounces";
const POUNDS: &str = "pounds";
const STONES: &str = "stones";
const TONNES: &str = "tonnes";

pub struct Mass;

impl Category for Mass {
    fn name() -> &'static str {
        "mass"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                CARATS,
                HashMap::from_iter([
                    (CARATS, identity as ConversionFunction),
                    (GRAMS, carats::to_grams),
                    (KILOGRAMS, carats::to_kilograms),
                    (MILLIGRAMS, carats::to_milligrams),
                    (OUNCES, carats::to_ounces),
                    (POUNDS, carats::to_pounds),
                    (STONES, carats::to_stones),
                    (TONNES, carats::to_tonnes),
                ]),
            ),
            (
                GRAMS,
                HashMap::from_iter([
                    (CARATS, grams::to_carats as ConversionFunction),
                    (GRAMS, identity),
                    (KILOGRAMS, grams::to_kilograms),
                    (MILLIGRAMS, grams::to_milligrams),
                    (OUNCES, grams::to_ounces),
                    (POUNDS, grams::to_pounds),
                    (STONES, grams::to_stones),
                    (TONNES, grams::to_tonnes),
                ]),
            ),
            (
                KILOGRAMS,
                HashMap::from_iter([
                    (CARATS, kilograms::to_carats as ConversionFunction),
                    (GRAMS, kilograms::to_grams),
                    (KILOGRAMS, identity),
                    (MILLIGRAMS, kilograms::to_milligrams),
                    (OUNCES, kilograms::to_ounces),
                    (POUNDS, kilograms::to_pounds),
                    (STONES, kilograms::to_stones),
                    (TONNES, kilograms::to_tonnes),
                ]),
            ),
            (
                MILLIGRAMS,
                HashMap::from_iter([
                    (CARATS, milligrams::to_carats as ConversionFunction),
                    (GRAMS, milligrams::to_grams),
                    (KILOGRAMS, milligrams::to_kilograms),
                    (MILLIGRAMS, identity),
                    (OUNCES, milligrams::to_ounces),
                    (POUNDS, milligrams::to_pounds),
                    (STONES, milligrams::to_stones),
                    (TONNES, milligrams::to_tonnes),
                ]),
            ),
            (
                OUNCES,
                HashMap::from_iter([
                    (CARATS, ounces::to_carats as ConversionFunction),
                    (GRAMS, ounces::to_grams),
                    (KILOGRAMS, ounces::to_kilograms),
                    (MILLIGRAMS, ounces::to_milligrams),
                    (OUNCES, identity),
                    (POUNDS, ounces::to_pounds),
                    (STONES, ounces::to_stones),
                    (TONNES, ounces::to_tonnes),
                ]),
            ),
            (
                POUNDS,
                HashMap::from_iter([
                    (CARATS, pounds::to_carats as ConversionFunction),
                    (GRAMS, pounds::to_grams),
                    (KILOGRAMS, pounds::to_kilograms),
                    (MILLIGRAMS, pounds::to_milligrams),
                    (OUNCES, pounds::to_ounces),
                    (POUNDS, identity),
                    (STONES, pounds::to_stones),
                    (TONNES, pounds::to_tonnes),
                ]),
            ),
            (
                "stones",
                HashMap::from_iter([
                    (CARATS, stones::to_carats as ConversionFunction),
                    (GRAMS, stones::to_grams),
                    (KILOGRAMS, stones::to_kilograms),
                    (MILLIGRAMS, stones::to_milligrams),
                    (OUNCES, stones::to_ounces),
                    (POUNDS, stones::to_pounds),
                    (STONES, identity),
                    (TONNES, stones::to_tonnes),
                ]),
            ),
            (
                TONNES,
                HashMap::from_iter([
                    (CARATS, tonnes::to_carats as ConversionFunction),
                    (GRAMS, tonnes::to_grams),
                    (KILOGRAMS, tonnes::to_kilograms),
                    (MILLIGRAMS, tonnes::to_milligrams),
                    (OUNCES, tonnes::to_ounces),
                    (POUNDS, tonnes::to_pounds),
                    (STONES, tonnes::to_stones),
                    (TONNES, identity),
                ]),
            ),
        ])
    }
}
