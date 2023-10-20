use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::pressure::*;

pub struct Pressure;

impl Values for Pressure {
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "atmospheres",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "bars",
                bars::to_atmospheres as BaseConversionFunction,
                atmospheres::to_bars as BaseConversionFunction,
            ),
            conversion("pascals", pascals::to_atmospheres, atmospheres::to_pascals),
            conversion("psi", psi::to_atmospheres, atmospheres::to_psi),
            conversion("torrs", torrs::to_atmospheres, atmospheres::to_torrs),
        ])
    }
}
