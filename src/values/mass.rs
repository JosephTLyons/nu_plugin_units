use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::mass::*;

pub struct Mass;

impl Values for Mass {
    fn name() -> &'static str {
        "mass"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "carats",
                HashMap::from_iter([
                    ("carats", identity as ConversionFunction),
                    ("grams", carats::to_grams),
                    ("kilograms", carats::to_kilograms),
                    ("milligrams", carats::to_milligrams),
                    ("ounces", carats::to_ounces),
                    ("pounds", carats::to_pounds),
                    ("stones", carats::to_stones),
                    ("tonnes", carats::to_tonnes),
                ]),
            ),
            (
                "grams",
                HashMap::from_iter([
                    ("carats", grams::to_carats as ConversionFunction),
                    ("grams", identity),
                    ("kilograms", grams::to_kilograms),
                    ("milligrams", grams::to_milligrams),
                    ("ounces", grams::to_ounces),
                    ("pounds", grams::to_pounds),
                    ("stones", grams::to_stones),
                    ("tonnes", grams::to_tonnes),
                ]),
            ),
            (
                "kilograms",
                HashMap::from_iter([
                    ("carats", kilograms::to_carats as ConversionFunction),
                    ("grams", kilograms::to_grams),
                    ("kilograms", identity),
                    ("milligrams", kilograms::to_milligrams),
                    ("ounces", kilograms::to_ounces),
                    ("pounds", kilograms::to_pounds),
                    ("stones", kilograms::to_stones),
                    ("tonnes", kilograms::to_tonnes),
                ]),
            ),
            (
                "milligrams",
                HashMap::from_iter([
                    ("carats", milligrams::to_carats as ConversionFunction),
                    ("grams", milligrams::to_grams),
                    ("kilograms", milligrams::to_kilograms),
                    ("milligrams", identity),
                    ("ounces", milligrams::to_ounces),
                    ("pounds", milligrams::to_pounds),
                    ("stones", milligrams::to_stones),
                    ("tonnes", milligrams::to_tonnes),
                ]),
            ),
            (
                "ounces",
                HashMap::from_iter([
                    ("carats", ounces::to_carats as ConversionFunction),
                    ("grams", ounces::to_grams),
                    ("kilograms", ounces::to_kilograms),
                    ("milligrams", ounces::to_milligrams),
                    ("ounces", identity),
                    ("pounds", ounces::to_pounds),
                    ("stones", ounces::to_stones),
                    ("tonnes", ounces::to_tonnes),
                ]),
            ),
            (
                "pounds",
                HashMap::from_iter([
                    ("carats", pounds::to_carats as ConversionFunction),
                    ("grams", pounds::to_grams),
                    ("kilograms", pounds::to_kilograms),
                    ("milligrams", pounds::to_milligrams),
                    ("ounces", pounds::to_ounces),
                    ("pounds", identity),
                    ("stones", pounds::to_stones),
                    ("tonnes", pounds::to_tonnes),
                ]),
            ),
            (
                "stones",
                HashMap::from_iter([
                    ("carats", stones::to_carats as ConversionFunction),
                    ("grams", stones::to_grams),
                    ("kilograms", stones::to_kilograms),
                    ("milligrams", stones::to_milligrams),
                    ("ounces", stones::to_ounces),
                    ("pounds", stones::to_pounds),
                    ("stones", identity),
                    ("tonnes", stones::to_tonnes),
                ]),
            ),
            (
                "tonnes",
                HashMap::from_iter([
                    ("carats", tonnes::to_carats as ConversionFunction),
                    ("grams", tonnes::to_grams),
                    ("kilograms", tonnes::to_kilograms),
                    ("milligrams", tonnes::to_milligrams),
                    ("ounces", tonnes::to_ounces),
                    ("pounds", tonnes::to_pounds),
                    ("stones", tonnes::to_stones),
                    ("tonnes", identity),
                ]),
            ),
        ])
    }
}
