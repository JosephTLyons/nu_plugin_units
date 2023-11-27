use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::pressure::*;

const ATMOSPHERES: &str = "atmospheres";
const BARS: &str = "bars";
const PASCALS: &str = "pascals";
const PSI: &str = "psi";
const TORRS: &str = "torrs";

pub struct Pressure;

impl Category for Pressure {
    fn name() -> &'static str {
        "pressure"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                ATMOSPHERES,
                HashMap::from_iter([
                    (ATMOSPHERES, identity as ConversionFunction),
                    (BARS, atmospheres::to_bars),
                    (PASCALS, atmospheres::to_pascals),
                    (PSI, atmospheres::to_psi),
                    (TORRS, atmospheres::to_torrs),
                ]),
            ),
            (
                BARS,
                HashMap::from_iter([
                    (ATMOSPHERES, bars::to_atmospheres as ConversionFunction),
                    (BARS, identity),
                    (PASCALS, bars::to_pascals),
                    (PSI, bars::to_psi),
                    (TORRS, bars::to_torrs),
                ]),
            ),
            (
                PASCALS,
                HashMap::from_iter([
                    (ATMOSPHERES, pascals::to_atmospheres as ConversionFunction),
                    (BARS, pascals::to_bars),
                    (PASCALS, identity),
                    (PSI, pascals::to_psi),
                    (TORRS, pascals::to_torrs),
                ]),
            ),
            (
                PSI,
                HashMap::from_iter([
                    (ATMOSPHERES, psi::to_atmospheres as ConversionFunction),
                    (BARS, psi::to_bars),
                    (PASCALS, psi::to_pascals),
                    (PSI, identity),
                    (TORRS, psi::to_torrs),
                ]),
            ),
            (
                TORRS,
                HashMap::from_iter([
                    (ATMOSPHERES, torrs::to_atmospheres as ConversionFunction),
                    (BARS, torrs::to_bars),
                    (PASCALS, torrs::to_pascals),
                    (PSI, torrs::to_psi),
                    (TORRS, identity),
                ]),
            ),
        ])
    }
}
