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

pub trait Category {
    fn name() -> &'static str;
    fn units() -> Vec<&'static str> {
        let mut units: Vec<_> = Self::conversion_function_map().into_keys().collect();
        units.sort();
        units
    }
    fn values(unit: &str, value: f64) -> ValuesFunctionReturn {
        let conversion_function_map = Self::conversion_function_map();
        let Some(conversion_functions) = conversion_function_map.get(unit) else {
            bail!("{} is not a valid unit", unit)
        };

        let values: Vec<(String, f64)> = conversion_functions
            .iter()
            .map(|(unit, conversion_function)| (unit.to_string(), conversion_function(value)))
            .collect();

        Ok(values)
    }
    fn conversion_function_map() -> ConversionFunctionMap;
}

#[cfg(test)]
mod tests {
    #[test]
    fn ensure_data_is_in_correct_format() {
        use super::*;

        let conversion_function_maps: [HashMap<&str, HashMap<&str, fn(f64) -> f64>>; 17] = [
            angle::Angle::conversion_function_map(),
            area::Area::conversion_function_map(),
            data_storage::DataStorage::conversion_function_map(),
            data_transfer_rate::DataTransferRate::conversion_function_map(),
            energy::Energy::conversion_function_map(),
            force::Force::conversion_function_map(),
            frequency::Frequency::conversion_function_map(),
            fuel_economy::FuelEconomy::conversion_function_map(),
            length::Length::conversion_function_map(),
            luminous_energy::LuminousEnergy::conversion_function_map(),
            magnetomotive_force::MagnetomotiveForce::conversion_function_map(),
            mass::Mass::conversion_function_map(),
            pressure::Pressure::conversion_function_map(),
            speed::Speed::conversion_function_map(),
            temperature::Temperature::conversion_function_map(),
            time::Time::conversion_function_map(),
            volume::Volume::conversion_function_map(),
        ];

        // TODO: Rename these for loop variables to be more readable and make sense
        for conversion_function_map in conversion_function_maps {
            for (from_unit, conversion_functions) in conversion_function_map {
                let illegal_characters = ['_', ' '];
                let illegal_characters_text = illegal_characters
                    .iter()
                    .map(|character| format!("`{}`", character))
                    .collect::<Vec<_>>()
                    .join(", ");

                assert!(
                    !from_unit.contains(illegal_characters),
                    "Unit \"{}\" should not contain any of the following characters: {}. Use `-`",
                    from_unit,
                    illegal_characters_text
                );

                for (to_unit, _) in conversion_functions {
                    assert!(
                        !to_unit.contains(illegal_characters),
                        "Unit \"{}\" should not contain any of the following characters: {}. Use `-`",
                        to_unit,
                        illegal_characters_text
                    );
                }
            }
        }
    }
}
