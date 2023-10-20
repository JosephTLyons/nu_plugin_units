use super::{conversion, BaseConversionFunctionsMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::energy::*;

pub struct Energy;

impl Values for Energy {
    fn name() -> &'static str {
        "energy"
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap {
        HashMap::from_iter([
            conversion("btu", identity, identity),
            conversion("calories", calories::to_btu, btu::to_calories),
            conversion(
                "electronvolts",
                electronvolts::to_btu,
                btu::to_electronvolts,
            ),
            conversion("foot-pounds", foot_pounds::to_btu, btu::to_foot_pounds),
            conversion("joules", joules::to_btu, btu::to_joules),
            conversion("kilocalories", kilocalories::to_btu, btu::to_kilocalories),
            conversion("kilojoules", kilojoules::to_btu, btu::to_kilojoules),
            conversion(
                "kilowatt-hours",
                kilowatt_hours::to_btu,
                btu::to_kilowatt_hours,
            ),
            conversion("us-therms", u_s_therms::to_btu, btu::to_u_s_therms),
            conversion("watt-hours", watt_hours::to_btu, btu::to_watt_hours),
        ])
    }
}
