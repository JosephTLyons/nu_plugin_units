use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::force::*;

const DYNES: &str = "dynes";
const KILOGRAMFORCE: &str = "kilogramforce";
const NEWTONS: &str = "newtons";
const POUNDALS: &str = "poundals";

pub struct Force;

impl Category for Force {
    fn name() -> &'static str {
        "force"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                DYNES,
                HashMap::from_iter([
                    (DYNES, identity as ConversionFunction),
                    (KILOGRAMFORCE, dynes::to_kilogramforce),
                    (NEWTONS, dynes::to_newtons),
                    (POUNDALS, dynes::to_poundals),
                ]),
            ),
            (
                KILOGRAMFORCE,
                HashMap::from_iter([
                    (DYNES, kilogramforce::to_dynes as ConversionFunction),
                    (KILOGRAMFORCE, identity),
                    (NEWTONS, kilogramforce::to_newtons),
                    (POUNDALS, kilogramforce::to_poundals),
                ]),
            ),
            (
                NEWTONS,
                HashMap::from_iter([
                    (DYNES, newtons::to_dynes as ConversionFunction),
                    (KILOGRAMFORCE, newtons::to_kilogramforce),
                    (NEWTONS, identity),
                    (POUNDALS, newtons::to_poundals),
                ]),
            ),
            (
                POUNDALS,
                HashMap::from_iter([
                    (DYNES, poundals::to_dynes as ConversionFunction),
                    (KILOGRAMFORCE, poundals::to_kilogramforce),
                    (NEWTONS, poundals::to_newtons),
                    (POUNDALS, identity),
                ]),
            ),
        ])
    }
}
