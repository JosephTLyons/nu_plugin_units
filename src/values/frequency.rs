use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::frequency::*;

pub struct Frequency;

impl Values for Frequency {
    fn name() -> &'static str {
        "frequency"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "gigahertz",
                HashMap::from_iter([
                    ("gigahertz", identity as ConversionFunction),
                    ("hertz", gigahertz::to_hertz),
                    ("kilohertz", gigahertz::to_kilohertz),
                    ("megahertz", gigahertz::to_megahertz),
                ]),
            ),
            (
                "hertz",
                HashMap::from_iter([
                    ("gigahertz", hertz::to_gigahertz as ConversionFunction),
                    ("hertz", identity),
                    ("kilohertz", hertz::to_kilohertz),
                    ("megahertz", hertz::to_megahertz),
                ]),
            ),
            (
                "kilohertz",
                HashMap::from_iter([
                    ("gigahertz", kilohertz::to_gigahertz as ConversionFunction),
                    ("hertz", kilohertz::to_hertz),
                    ("kilohertz", identity),
                    ("megahertz", kilohertz::to_megahertz),
                ]),
            ),
            (
                "megahertz",
                HashMap::from_iter([
                    ("gigahertz", megahertz::to_gigahertz as ConversionFunction),
                    ("hertz", megahertz::to_hertz),
                    ("kilohertz", megahertz::to_kilohertz),
                    ("megahertz", identity),
                ]),
            ),
        ])
    }
}
