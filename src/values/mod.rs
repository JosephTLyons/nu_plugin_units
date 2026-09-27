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

pub type ConversionFunction = fn(f64) -> f64;
/// Maps each unit to the functions that convert it into every unit of the same category.
pub type ConversionFunctionMap = HashMap<&'static str, HashMap<&'static str, ConversionFunction>>;
pub type ConversionFunctionMapBuilder = fn() -> ConversionFunctionMap;

pub trait Category {
    const NAME: &'static str;
    fn conversion_function_map() -> ConversionFunctionMap;
}

/// Every category the plugin supports, as (name, conversion table builder) pairs.
pub const CATEGORIES: &[(&str, ConversionFunctionMapBuilder)] = &[
    (Angle::NAME, Angle::conversion_function_map),
    (Area::NAME, Area::conversion_function_map),
    (DataStorage::NAME, DataStorage::conversion_function_map),
    (
        DataTransferRate::NAME,
        DataTransferRate::conversion_function_map,
    ),
    (Energy::NAME, Energy::conversion_function_map),
    (Force::NAME, Force::conversion_function_map),
    (Frequency::NAME, Frequency::conversion_function_map),
    (FuelEconomy::NAME, FuelEconomy::conversion_function_map),
    (Length::NAME, Length::conversion_function_map),
    (
        LuminousEnergy::NAME,
        LuminousEnergy::conversion_function_map,
    ),
    (
        MagnetomotiveForce::NAME,
        MagnetomotiveForce::conversion_function_map,
    ),
    (Mass::NAME, Mass::conversion_function_map),
    (Pressure::NAME, Pressure::conversion_function_map),
    (Speed::NAME, Speed::conversion_function_map),
    (Temperature::NAME, Temperature::conversion_function_map),
    (Time::NAME, Time::conversion_function_map),
    (Volume::NAME, Volume::conversion_function_map),
];

pub enum ConversionError {
    UnknownCategory { valid_categories: Vec<&'static str> },
    UnknownUnit { valid_units: Vec<&'static str> },
}

/// Converts `value` from `unit` into every unit of `category`, sorted by unit name.
pub fn convert(
    category: &str,
    unit: &str,
    value: f64,
) -> Result<Vec<(&'static str, f64)>, ConversionError> {
    let Some((_, conversion_function_map)) = CATEGORIES.iter().find(|(name, _)| *name == category)
    else {
        let mut valid_categories: Vec<_> = CATEGORIES.iter().map(|(name, _)| *name).collect();
        valid_categories.sort();
        return Err(ConversionError::UnknownCategory { valid_categories });
    };

    let conversion_function_map = conversion_function_map();

    let Some(conversion_functions) = conversion_function_map.get(unit) else {
        let mut valid_units: Vec<_> = conversion_function_map.keys().copied().collect();
        valid_units.sort();
        return Err(ConversionError::UnknownUnit { valid_units });
    };

    let mut values: Vec<_> = conversion_functions
        .iter()
        .map(|(unit, conversion_function)| (*unit, conversion_function(value)))
        .collect();
    values.sort_by_key(|(unit, _)| *unit);

    Ok(values)
}

#[cfg(test)]
mod tests {
    #[test]
    fn ensure_data_is_in_correct_format() {
        use super::*;

        // TODO: Rename these for loop variables to be more readable and make sense
        for (_, conversion_function_map) in CATEGORIES {
            for (from_unit, conversion_functions) in conversion_function_map() {
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
