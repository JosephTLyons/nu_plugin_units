use std::collections::HashMap;

mod angle;
mod area;
mod data_storage;
mod data_transfer_rate;
mod energy;
mod force;
mod frequency;
mod fuel_economy;
mod length;
mod luminous_energy;
mod magnetomotive_force;
mod mass;
mod pressure;
mod speed;
mod temperature;
mod time;
mod volume;

pub use angle::Angle;
pub use area::Area;
pub use data_storage::DataStorage;
pub use data_transfer_rate::DataTransferRate;
pub use energy::Energy;
pub use force::Force;
pub use frequency::Frequency;
pub use fuel_economy::FuelEconomy;
pub use length::Length;
pub use luminous_energy::LuminousEnergy;
pub use magnetomotive_force::MagnetomotiveForce;
pub use mass::Mass;
pub use pressure::Pressure;
pub use speed::Speed;
pub use temperature::Temperature;
pub use time::Time;
pub use volume::Volume;

use anyhow::{anyhow, Result};

pub type BaseConversionFunction = fn(f64) -> f64;
pub type BaseConversionFunctionsMap =
    HashMap<&'static str, (BaseConversionFunction, BaseConversionFunction)>;
pub type ValuesFunction = fn(&str, f64) -> anyhow::Result<Vec<(String, f64)>>;

// TODO: Rename? Maybe this should be dimension?
pub trait Values {
    fn name() -> &'static str;
    fn values(unit: &str, value: f64) -> Result<Vec<(String, f64)>> {
        let base_conversion_functions = Self::base_conversion_functions();

        let Some(conversion_functions) = base_conversion_functions.get(unit) else {
            return Err(anyhow!("{} is not a valid unit", unit));
        };

        let (to_base, _) = conversion_functions;
        let base_value = to_base(value);

        let values: Vec<(String, f64)> = base_conversion_functions
            .iter()
            .map(|(unit, conversion_functions)| {
                // A few asserts to make sure data is stored in a consistent way
                assert!(!unit.contains('_'), "Unit should not contain underscores");
                assert!(!unit.contains(' '), "Unit should not contain spaces");

                let (_, from_base) = conversion_functions;
                let value = from_base(base_value);

                (unit.to_string(), value)
            })
            .collect();

        Ok(values)
    }
    fn base_conversion_functions() -> BaseConversionFunctionsMap;
}

// TODO: Rename
pub fn conversion(
    name: &'static str,
    a: BaseConversionFunction,
    b: BaseConversionFunction,
) -> (
    &'static str,
    (BaseConversionFunction, BaseConversionFunction),
) {
    (name, (a, b))
}
