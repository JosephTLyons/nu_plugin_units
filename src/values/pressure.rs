use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::pressure::*;

pub struct Pressure;

impl Values for Pressure {
    fn name() -> &'static str {
        "pressure"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "atmospheres",
                HashMap::from_iter([
                    ("atmospheres", identity as ConversionFunction),
                    ("bars", atmospheres::to_bars),
                    ("pascals", atmospheres::to_pascals),
                    ("psi", atmospheres::to_psi),
                    ("torrs", atmospheres::to_torrs),
                ]),
            ),
            (
                "bars",
                HashMap::from_iter([
                    ("atmospheres", bars::to_atmospheres as ConversionFunction),
                    ("bars", identity),
                    ("pascals", bars::to_pascals),
                    ("psi", bars::to_psi),
                    ("torrs", bars::to_torrs),
                ]),
            ),
            (
                "pascals",
                HashMap::from_iter([
                    ("atmospheres", pascals::to_atmospheres as ConversionFunction),
                    ("bars", pascals::to_bars),
                    ("pascals", identity),
                    ("psi", pascals::to_psi),
                    ("torrs", pascals::to_torrs),
                ]),
            ),
            (
                "psi",
                HashMap::from_iter([
                    ("atmospheres", psi::to_atmospheres as ConversionFunction),
                    ("bars", psi::to_bars),
                    ("pascals", psi::to_pascals),
                    ("psi", identity),
                    ("torrs", psi::to_torrs),
                ]),
            ),
            (
                "torrs",
                HashMap::from_iter([
                    ("atmospheres", torrs::to_atmospheres as ConversionFunction),
                    ("bars", torrs::to_bars),
                    ("pascals", torrs::to_pascals),
                    ("psi", torrs::to_psi),
                    ("torrs", identity),
                ]),
            ),
        ])
    }
}
