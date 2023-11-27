use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::frequency::*;

const GIGAHERTZ: &str = "gigahertz";
const HERTZ: &str = "hertz";
const KILOHERTZ: &str = "kilohertz";
const MEGAHERTZ: &str = "megahertz";

pub struct Frequency;

impl Category for Frequency {
    fn name() -> &'static str {
        "frequency"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                GIGAHERTZ,
                HashMap::from_iter([
                    (GIGAHERTZ, identity as ConversionFunction),
                    (HERTZ, gigahertz::to_hertz),
                    (KILOHERTZ, gigahertz::to_kilohertz),
                    (MEGAHERTZ, gigahertz::to_megahertz),
                ]),
            ),
            (
                HERTZ,
                HashMap::from_iter([
                    (GIGAHERTZ, hertz::to_gigahertz as ConversionFunction),
                    (HERTZ, identity),
                    (KILOHERTZ, hertz::to_kilohertz),
                    (MEGAHERTZ, hertz::to_megahertz),
                ]),
            ),
            (
                KILOHERTZ,
                HashMap::from_iter([
                    (GIGAHERTZ, kilohertz::to_gigahertz as ConversionFunction),
                    (HERTZ, kilohertz::to_hertz),
                    (KILOHERTZ, identity),
                    (MEGAHERTZ, kilohertz::to_megahertz),
                ]),
            ),
            (
                MEGAHERTZ,
                HashMap::from_iter([
                    (GIGAHERTZ, megahertz::to_gigahertz as ConversionFunction),
                    (HERTZ, megahertz::to_hertz),
                    (KILOHERTZ, megahertz::to_kilohertz),
                    (MEGAHERTZ, identity),
                ]),
            ),
        ])
    }
}
