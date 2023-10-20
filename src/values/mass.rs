use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::mass::*;

pub struct Mass;

impl Values for Mass {
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "carats",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "grams",
                grams::to_carats as BaseConversionFunction,
                carats::to_grams as BaseConversionFunction,
            ),
            conversion("kilograms", kilograms::to_carats, carats::to_kilograms),
            conversion("milligrams", milligrams::to_carats, carats::to_milligrams),
            conversion("ounces", ounces::to_carats, carats::to_ounces),
            conversion("pounds", pounds::to_carats, carats::to_pounds),
            conversion("stones", stones::to_carats, carats::to_stones),
            conversion("tonnes", tonnes::to_carats, carats::to_tonnes),
        ])
    }
}
