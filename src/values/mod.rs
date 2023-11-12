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

use anyhow::bail;

// Rename or get rid of these all together - they might only be making things more confusing
pub type ConversionFunction = fn(f64) -> f64;
pub type ConversionFunctionMap = HashMap<&'static str, HashMap<&'static str, ConversionFunction>>;
pub type ValuesFunctionReturn = anyhow::Result<Vec<(String, f64)>>;
pub type ValuesFunction = fn(&str, f64) -> ValuesFunctionReturn;

// TODO: Rename? Maybe this should be dimension?
pub trait Values {
    fn name() -> &'static str;
    fn units() -> Vec<&'static str> {
        let mut units: Vec<_> = Self::hash_map().into_keys().collect();
        units.sort();
        units
    }
    fn values(unit: &str, value: f64) -> ValuesFunctionReturn {
        let hash_map = Self::hash_map();
        let Some(conversion_functions) = hash_map.get(unit) else {
            bail!("{} is not a valid unit", unit)
        };

        let values: Vec<(String, f64)> = conversion_functions
            .iter()
            .map(|(unit, conversion_function)| {
                // Make sure data is stored in a consistent way
                // TODO: Pull illegal_characters out into a variable
                if unit.contains(['_', ' ']) {
                    eprintln!("Unit \"{}\" should not contain `_` or ` `. Use `-`", unit);
                }

                (unit.to_string(), conversion_function(value))
            })
            .collect();

        Ok(values)
    }
    fn hash_map() -> ConversionFunctionMap;
}
