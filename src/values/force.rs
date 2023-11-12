use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::force::*;

pub struct Force;

impl Values for Force {
    fn name() -> &'static str {
        "force"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "dynes",
                HashMap::from_iter([
                    ("dynes", identity as ConversionFunction),
                    ("kilogramforce", dynes::to_kilogramforce),
                    ("newtons", dynes::to_newtons),
                    ("poundals", dynes::to_poundals),
                ]),
            ),
            (
                "kilogramforce",
                HashMap::from_iter([
                    ("dynes", kilogramforce::to_dynes as ConversionFunction),
                    ("kilogramforce", identity),
                    ("newtons", kilogramforce::to_newtons),
                    ("poundals", kilogramforce::to_poundals),
                ]),
            ),
            (
                "newtons",
                HashMap::from_iter([
                    ("dynes", newtons::to_dynes as ConversionFunction),
                    ("kilogramforce", newtons::to_kilogramforce),
                    ("newtons", identity),
                    ("poundals", newtons::to_poundals),
                ]),
            ),
            (
                "poundals",
                HashMap::from_iter([
                    ("dynes", poundals::to_dynes as ConversionFunction),
                    ("kilogramforce", poundals::to_kilogramforce),
                    ("newtons", poundals::to_newtons),
                    ("poundals", identity),
                ]),
            ),
        ])
    }
}
