use super::{ConversionFunction, ConversionFunctionMap, Values};
use std::{collections::HashMap, convert::identity};
use unit_conversions::energy::*;

pub struct Energy;

impl Values for Energy {
    fn name() -> &'static str {
        "energy"
    }
    fn hash_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                "btu",
                HashMap::from_iter([
                    ("btu", identity as ConversionFunction),
                    ("calories", btu::to_calories),
                    ("electronvolts", btu::to_electronvolts),
                    ("foot-pounds", btu::to_foot_pounds),
                    ("joules", btu::to_joules),
                    ("kilocalories", btu::to_kilocalories),
                    ("kilojoules", btu::to_kilojoules),
                    ("kilowatt-hours", btu::to_kilowatt_hours),
                    ("us-therms", btu::to_u_s_therms),
                    ("watt-hours", btu::to_watt_hours),
                ]),
            ),
            (
                "calories",
                HashMap::from_iter([
                    ("btu", calories::to_btu as ConversionFunction),
                    ("calories", identity),
                    ("electronvolts", calories::to_electronvolts),
                    ("foot-pounds", calories::to_foot_pounds),
                    ("joules", calories::to_joules),
                    ("kilocalories", calories::to_kilocalories),
                    ("kilojoules", calories::to_kilojoules),
                    ("kilowatt-hours", calories::to_kilowatt_hours),
                    ("us-therms", calories::to_u_s_therms),
                    ("watt-hours", calories::to_watt_hours),
                ]),
            ),
            (
                "electronvolts",
                HashMap::from_iter([
                    ("btu", electronvolts::to_btu as ConversionFunction),
                    ("calories", electronvolts::to_calories),
                    ("electronvolts", identity),
                    ("foot-pounds", electronvolts::to_foot_pounds),
                    ("joules", electronvolts::to_joules),
                    ("kilocalories", electronvolts::to_kilocalories),
                    ("kilojoules", electronvolts::to_kilojoules),
                    ("kilowatt-hours", electronvolts::to_kilowatt_hours),
                    ("us-therms", electronvolts::to_u_s_therms),
                    ("watt-hours", electronvolts::to_watt_hours),
                ]),
            ),
            (
                "foot-pounds",
                HashMap::from_iter([
                    ("btu", foot_pounds::to_btu as ConversionFunction),
                    ("calories", foot_pounds::to_calories),
                    ("electronvolts", foot_pounds::to_electronvolts),
                    ("foot-pounds", identity),
                    ("joules", foot_pounds::to_joules),
                    ("kilocalories", foot_pounds::to_kilocalories),
                    ("kilojoules", foot_pounds::to_kilojoules),
                    ("kilowatt-hours", foot_pounds::to_kilowatt_hours),
                    ("us-therms", foot_pounds::to_u_s_therms),
                    ("watt-hours", foot_pounds::to_watt_hours),
                ]),
            ),
            (
                "joules",
                HashMap::from_iter([
                    ("btu", joules::to_btu as ConversionFunction),
                    ("calories", joules::to_calories),
                    ("electronvolts", joules::to_electronvolts),
                    ("foot-pounds", joules::to_foot_pounds),
                    ("joules", identity),
                    ("kilocalories", joules::to_kilocalories),
                    ("kilojoules", joules::to_kilojoules),
                    ("kilowatt-hours", joules::to_kilowatt_hours),
                    ("us-therms", joules::to_u_s_therms),
                    ("watt-hours", joules::to_watt_hours),
                ]),
            ),
            (
                "kilocalories",
                HashMap::from_iter([
                    ("btu", kilocalories::to_btu as ConversionFunction),
                    ("calories", kilocalories::to_calories),
                    ("electronvolts", kilocalories::to_electronvolts),
                    ("foot-pounds", kilocalories::to_foot_pounds),
                    ("joules", kilocalories::to_joules),
                    ("kilocalories", identity),
                    ("kilojoules", kilocalories::to_kilojoules),
                    ("kilowatt-hours", kilocalories::to_kilowatt_hours),
                    ("us-therms", kilocalories::to_u_s_therms),
                    ("watt-hours", kilocalories::to_watt_hours),
                ]),
            ),
            (
                "kilojoules",
                HashMap::from_iter([
                    ("btu", kilojoules::to_btu as ConversionFunction),
                    ("calories", kilojoules::to_calories),
                    ("electronvolts", kilojoules::to_electronvolts),
                    ("foot-pounds", kilojoules::to_foot_pounds),
                    ("joules", kilojoules::to_joules),
                    ("kilocalories", kilojoules::to_kilocalories),
                    ("kilojoules", identity),
                    ("kilowatt-hours", kilojoules::to_kilowatt_hours),
                    ("us-therms", kilojoules::to_u_s_therms),
                    ("watt-hours", kilojoules::to_watt_hours),
                ]),
            ),
            (
                "kilowatt-hours",
                HashMap::from_iter([
                    ("btu", kilowatt_hours::to_btu as ConversionFunction),
                    ("calories", kilowatt_hours::to_calories),
                    ("electronvolts", kilowatt_hours::to_electronvolts),
                    ("foot-pounds", kilowatt_hours::to_foot_pounds),
                    ("joules", kilowatt_hours::to_joules),
                    ("kilocalories", kilowatt_hours::to_kilocalories),
                    ("kilojoules", kilowatt_hours::to_kilojoules),
                    ("kilowatt-hours", identity),
                    ("us-therms", kilowatt_hours::to_u_s_therms),
                    ("watt-hours", kilowatt_hours::to_watt_hours),
                ]),
            ),
            (
                "us-therms",
                HashMap::from_iter([
                    ("btu", u_s_therms::to_btu as ConversionFunction),
                    ("calories", u_s_therms::to_calories),
                    ("electronvolts", u_s_therms::to_electronvolts),
                    ("foot-pounds", u_s_therms::to_foot_pounds),
                    ("joules", u_s_therms::to_joules),
                    ("kilocalories", u_s_therms::to_kilocalories),
                    ("kilojoules", u_s_therms::to_kilojoules),
                    ("kilowatt-hours", u_s_therms::to_kilowatt_hours),
                    ("us-therms", identity),
                    ("watt-hours", u_s_therms::to_watt_hours),
                ]),
            ),
            (
                "watt-hours",
                HashMap::from_iter([
                    ("btu", watt_hours::to_btu as ConversionFunction),
                    ("calories", watt_hours::to_calories),
                    ("electronvolts", watt_hours::to_electronvolts),
                    ("foot-pounds", watt_hours::to_foot_pounds),
                    ("joules", watt_hours::to_joules),
                    ("kilocalories", watt_hours::to_kilocalories),
                    ("kilojoules", watt_hours::to_kilojoules),
                    ("kilowatt-hours", watt_hours::to_kilowatt_hours),
                    ("us-therms", watt_hours::to_u_s_therms),
                    ("watt-hours", identity),
                ]),
            ),
        ])
    }
}
