use super::{Category, ConversionFunction, ConversionFunctionMap};
use std::{collections::HashMap, convert::identity};
use unit_conversions::energy::*;

const BTU: &str = "btu";
const CALORIES: &str = "calories";
const ELECTRONVOLTS: &str = "electronvolts";
const FOOT_POUNDS: &str = "foot-pounds";
const JOULES: &str = "joules";
const KILOCALORIES: &str = "kilocalories";
const KILOJOULES: &str = "kilojoules";
const KILOWATT_HOURS: &str = "kilowatt-hours";
const US_THERMS: &str = "us-therms";
const WATT_HOURS: &str = "watt-hours";

pub struct Energy;

impl Category for Energy {
    fn name() -> &'static str {
        "energy"
    }
    fn conversion_function_map() -> ConversionFunctionMap {
        HashMap::from_iter([
            (
                BTU,
                HashMap::from_iter([
                    (BTU, identity as ConversionFunction),
                    (CALORIES, btu::to_calories),
                    (ELECTRONVOLTS, btu::to_electronvolts),
                    (FOOT_POUNDS, btu::to_foot_pounds),
                    (JOULES, btu::to_joules),
                    (KILOCALORIES, btu::to_kilocalories),
                    (KILOJOULES, btu::to_kilojoules),
                    (KILOWATT_HOURS, btu::to_kilowatt_hours),
                    (US_THERMS, btu::to_u_s_therms),
                    (WATT_HOURS, btu::to_watt_hours),
                ]),
            ),
            (
                CALORIES,
                HashMap::from_iter([
                    (BTU, calories::to_btu as ConversionFunction),
                    (CALORIES, identity),
                    (ELECTRONVOLTS, calories::to_electronvolts),
                    (FOOT_POUNDS, calories::to_foot_pounds),
                    (JOULES, calories::to_joules),
                    (KILOCALORIES, calories::to_kilocalories),
                    (KILOJOULES, calories::to_kilojoules),
                    (KILOWATT_HOURS, calories::to_kilowatt_hours),
                    (US_THERMS, calories::to_u_s_therms),
                    (WATT_HOURS, calories::to_watt_hours),
                ]),
            ),
            (
                ELECTRONVOLTS,
                HashMap::from_iter([
                    (BTU, electronvolts::to_btu as ConversionFunction),
                    (CALORIES, electronvolts::to_calories),
                    (ELECTRONVOLTS, identity),
                    (FOOT_POUNDS, electronvolts::to_foot_pounds),
                    (JOULES, electronvolts::to_joules),
                    (KILOCALORIES, electronvolts::to_kilocalories),
                    (KILOJOULES, electronvolts::to_kilojoules),
                    (KILOWATT_HOURS, electronvolts::to_kilowatt_hours),
                    (US_THERMS, electronvolts::to_u_s_therms),
                    (WATT_HOURS, electronvolts::to_watt_hours),
                ]),
            ),
            (
                FOOT_POUNDS,
                HashMap::from_iter([
                    (BTU, foot_pounds::to_btu as ConversionFunction),
                    (CALORIES, foot_pounds::to_calories),
                    (ELECTRONVOLTS, foot_pounds::to_electronvolts),
                    (FOOT_POUNDS, identity),
                    (JOULES, foot_pounds::to_joules),
                    (KILOCALORIES, foot_pounds::to_kilocalories),
                    (KILOJOULES, foot_pounds::to_kilojoules),
                    (KILOWATT_HOURS, foot_pounds::to_kilowatt_hours),
                    (US_THERMS, foot_pounds::to_u_s_therms),
                    (WATT_HOURS, foot_pounds::to_watt_hours),
                ]),
            ),
            (
                JOULES,
                HashMap::from_iter([
                    (BTU, joules::to_btu as ConversionFunction),
                    (CALORIES, joules::to_calories),
                    (ELECTRONVOLTS, joules::to_electronvolts),
                    (FOOT_POUNDS, joules::to_foot_pounds),
                    (JOULES, identity),
                    (KILOCALORIES, joules::to_kilocalories),
                    (KILOJOULES, joules::to_kilojoules),
                    (KILOWATT_HOURS, joules::to_kilowatt_hours),
                    (US_THERMS, joules::to_u_s_therms),
                    (WATT_HOURS, joules::to_watt_hours),
                ]),
            ),
            (
                KILOCALORIES,
                HashMap::from_iter([
                    (BTU, kilocalories::to_btu as ConversionFunction),
                    (CALORIES, kilocalories::to_calories),
                    (ELECTRONVOLTS, kilocalories::to_electronvolts),
                    (FOOT_POUNDS, kilocalories::to_foot_pounds),
                    (JOULES, kilocalories::to_joules),
                    (KILOCALORIES, identity),
                    (KILOJOULES, kilocalories::to_kilojoules),
                    (KILOWATT_HOURS, kilocalories::to_kilowatt_hours),
                    (US_THERMS, kilocalories::to_u_s_therms),
                    (WATT_HOURS, kilocalories::to_watt_hours),
                ]),
            ),
            (
                "kilojoules",
                HashMap::from_iter([
                    (BTU, kilojoules::to_btu as ConversionFunction),
                    (CALORIES, kilojoules::to_calories),
                    (ELECTRONVOLTS, kilojoules::to_electronvolts),
                    (FOOT_POUNDS, kilojoules::to_foot_pounds),
                    (JOULES, kilojoules::to_joules),
                    (KILOCALORIES, kilojoules::to_kilocalories),
                    (KILOJOULES, identity),
                    (KILOWATT_HOURS, kilojoules::to_kilowatt_hours),
                    (US_THERMS, kilojoules::to_u_s_therms),
                    (WATT_HOURS, kilojoules::to_watt_hours),
                ]),
            ),
            (
                KILOWATT_HOURS,
                HashMap::from_iter([
                    (BTU, kilowatt_hours::to_btu as ConversionFunction),
                    (CALORIES, kilowatt_hours::to_calories),
                    (ELECTRONVOLTS, kilowatt_hours::to_electronvolts),
                    (FOOT_POUNDS, kilowatt_hours::to_foot_pounds),
                    (JOULES, kilowatt_hours::to_joules),
                    (KILOCALORIES, kilowatt_hours::to_kilocalories),
                    (KILOJOULES, kilowatt_hours::to_kilojoules),
                    (KILOWATT_HOURS, identity),
                    (US_THERMS, kilowatt_hours::to_u_s_therms),
                    (WATT_HOURS, kilowatt_hours::to_watt_hours),
                ]),
            ),
            (
                US_THERMS,
                HashMap::from_iter([
                    (BTU, u_s_therms::to_btu as ConversionFunction),
                    (CALORIES, u_s_therms::to_calories),
                    (ELECTRONVOLTS, u_s_therms::to_electronvolts),
                    (FOOT_POUNDS, u_s_therms::to_foot_pounds),
                    (JOULES, u_s_therms::to_joules),
                    (KILOCALORIES, u_s_therms::to_kilocalories),
                    (KILOJOULES, u_s_therms::to_kilojoules),
                    (KILOWATT_HOURS, u_s_therms::to_kilowatt_hours),
                    (US_THERMS, identity),
                    (WATT_HOURS, u_s_therms::to_watt_hours),
                ]),
            ),
            (
                WATT_HOURS,
                HashMap::from_iter([
                    (BTU, watt_hours::to_btu as ConversionFunction),
                    (CALORIES, watt_hours::to_calories),
                    (ELECTRONVOLTS, watt_hours::to_electronvolts),
                    (FOOT_POUNDS, watt_hours::to_foot_pounds),
                    (JOULES, watt_hours::to_joules),
                    (KILOCALORIES, watt_hours::to_kilocalories),
                    (KILOJOULES, watt_hours::to_kilojoules),
                    (KILOWATT_HOURS, watt_hours::to_kilowatt_hours),
                    (US_THERMS, watt_hours::to_u_s_therms),
                    (WATT_HOURS, identity),
                ]),
            ),
        ])
    }
}
