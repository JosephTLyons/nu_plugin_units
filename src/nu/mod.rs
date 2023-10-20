use crate::values::*;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, Record, SyntaxShape, Value};

// TODO: Move
enum Dimensions {
    Angle,
    Area,
    DataStorage,
    DataTransferRate,
    Energy,
    Force,
    Frequency,
    FuelEconomy,
    Length,
    LuminousEnergy,
    MagnetomotiveForce,
    Mass,
    Pressure,
    Speed,
    Temperature,
    Time,
    Volume,
}

pub struct Units;

// TODO: Is the term dimension the correct thing to use here?
// TODO: Is it possible to force a list of options for dimensions and units?
impl Plugin for Units {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("units")
            .usage("Convert between units")
            .required_named(
                "dimension",
                SyntaxShape::String,
                "specify the dimension",
                Some('d'),
            )
            .required_named(
                "unit",
                SyntaxShape::String,
                "specify the unit type",
                Some('u'),
            )
            .required_named("value", SyntaxShape::Float, "specify the value", Some('v'))
            .category(Category::Generators)]
    }

    fn run(&mut self, _: &str, call: &EvaluatedCall, _: &Value) -> Result<Value, LabeledError> {
        let tag = call.head;

        let dimension = call.get_flag::<String>("dimension");
        let unit = call.get_flag::<String>("unit");
        let value = call.get_flag::<f64>("value");

        let Ok(Some(dimension)) = dimension else {
            // TODO: Return error instead
            return Ok(Value::nothing(tag));
        };

        let (Ok(Some(unit)), Ok(Some(value))) = (unit, value) else {
            // TODO: Return error instead
            return Ok(Value::nothing(tag));
        };

        let values_function = match dimension.as_ref() {
            "angle" => Angle::values,
            "area" => Area::values,
            "data-storage" => DataStorage::values,
            "data-transfer-rate" => DataTransferRate::values,
            "energy" => Energy::values,
            "force" => Force::values,
            "frequency" => Frequency::values,
            "fuel-economy" => FuelEconomy::values,
            "length" => Length::values,
            "luminous-energy" => LuminousEnergy::values,
            "magnetomotive-force" => MagnetomotiveForce::values,
            "mass" => Mass::values,
            "pressure" => Pressure::values,
            "speed" => Speed::values,
            "temperature" => Temperature::values,
            "time" => Time::values,
            "volume" => Volume::values,
            _ => return Ok(Value::nothing(tag)), // Should be an error
        };

        let Some(mut values) = values_function(&unit, value) else {
            // TODO: Return error instead
            return Ok(Value::nothing(tag));
        };

        // TODO: Avoid clone
        values.sort_by_key(|value| value.0.clone());

        let values: Vec<_> = values
            .iter()
            .map(|(unit, value)| {
                let unit = unit.replace('-', " ");
                let record = Record::from_iter([
                    ("unit".into(), Value::string(unit, tag)),
                    ("value".into(), Value::float(*value, tag)),
                ]);
                Value::record(record, tag)
            })
            .collect();

        Ok(Value::list(values, tag))
    }
}

// Errors that are produced by picking a wrong dimension or unit should show the list of available. Find a reuse the hashmaps to surface those.

// TODO: Reasses which units to use as base units
