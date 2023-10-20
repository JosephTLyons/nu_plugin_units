use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::pressure::*;

pub struct Pressure;

impl Values for Pressure {
    fn name() -> &'static str {
        "pressure"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("atmospheres", identity, identity),
            conversion("bars", bars::to_atmospheres, atmospheres::to_bars),
            conversion("pascals", pascals::to_atmospheres, atmospheres::to_pascals),
            conversion("psi", psi::to_atmospheres, atmospheres::to_psi),
            conversion("torrs", torrs::to_atmospheres, atmospheres::to_torrs),
        ])
    }
}
