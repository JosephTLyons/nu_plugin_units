use super::{conversion, BaseConversionFunction, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::energy::*;

pub struct Energy;

impl Values for Energy {
    fn name() -> &'static str {
        "energy"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion(
                "btu",
                identity as BaseConversionFunction,
                identity as BaseConversionFunction,
            ),
            conversion(
                "calories",
                calories::to_btu as BaseConversionFunction,
                btu::to_calories as BaseConversionFunction,
            ),
            conversion(
                "electronvolts",
                electronvolts::to_btu as BaseConversionFunction,
                btu::to_electronvolts as BaseConversionFunction,
            ),
            conversion(
                "foot-pounds",
                foot_pounds::to_btu as BaseConversionFunction,
                btu::to_foot_pounds as BaseConversionFunction,
            ),
            conversion(
                "joules",
                joules::to_btu as BaseConversionFunction,
                btu::to_joules as BaseConversionFunction,
            ),
            conversion(
                "kilocalories",
                kilocalories::to_btu as BaseConversionFunction,
                btu::to_kilocalories as BaseConversionFunction,
            ),
            conversion(
                "kilojoules",
                kilojoules::to_btu as BaseConversionFunction,
                btu::to_kilojoules as BaseConversionFunction,
            ),
            conversion(
                "kilowatt-hours",
                kilowatt_hours::to_btu as BaseConversionFunction,
                btu::to_kilowatt_hours as BaseConversionFunction,
            ),
            conversion(
                "us-therms",
                u_s_therms::to_btu as BaseConversionFunction,
                btu::to_u_s_therms as BaseConversionFunction,
            ),
            conversion(
                "watt-hours",
                watt_hours::to_btu as BaseConversionFunction,
                btu::to_watt_hours as BaseConversionFunction,
            ),
        ])
    }
}
