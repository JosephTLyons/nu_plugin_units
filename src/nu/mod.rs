use std::collections::HashMap;

use crate::values::*;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, Record, SyntaxShape, Value};

pub struct Units;

const DIMENSION_FLAG_NAME: &'static str = "dimension";
const UNIT_FLAG_NAME: &'static str = "unit";
const VALUE_FLAG_NAME: &'static str = "value";

// TODO: Is the term dimension the correct thing to use here?
impl Plugin for Units {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("units")
            .usage("Convert between units")
            .required_named(
                DIMENSION_FLAG_NAME,
                SyntaxShape::String,
                "specify the dimension",
                Some('d'),
            )
            .required_named(
                UNIT_FLAG_NAME,
                SyntaxShape::String,
                "specify the unit type",
                Some('u'),
            )
            .required_named(
                VALUE_FLAG_NAME,
                SyntaxShape::Float,
                "specify the value",
                Some('v'),
            )
            .category(Category::Generators)]
    }

    fn run(&mut self, _: &str, call: &EvaluatedCall, _: &Value) -> Result<Value, LabeledError> {
        let tag = call.head;

        // The `unwrap()`s are safe, since the flags, arguments, and arguments types are enforced by the signature
        // The `unwrap()`s here are to make sure the strings looked up matches the ones in the signature
        let dimension = call.get_flag_value(DIMENSION_FLAG_NAME).unwrap();
        let dimension_span = dimension.span();
        let dimension = dimension.as_string().unwrap();

        let unit = call.get_flag_value(UNIT_FLAG_NAME).unwrap();
        let unit_span = unit.span();
        let unit = unit.as_string().unwrap();

        let dimensions: HashMap<_, _> = HashMap::from_iter([
            hash_map_tuple(Angle),
            hash_map_tuple(Area),
            hash_map_tuple(DataStorage),
            hash_map_tuple(DataTransferRate),
            hash_map_tuple(Energy),
            hash_map_tuple(Force),
            hash_map_tuple(Frequency),
            hash_map_tuple(FuelEconomy),
            hash_map_tuple(Length),
            hash_map_tuple(LuminousEnergy),
            hash_map_tuple(MagnetomotiveForce),
            hash_map_tuple(Mass),
            hash_map_tuple(Pressure),
            hash_map_tuple(Speed),
            hash_map_tuple(Temperature),
            hash_map_tuple(Time),
            hash_map_tuple(Volume),
        ]);

        let Some((values_function, units)) = dimensions.get(dimension.as_str()) else {
            let mut valid_dimensions = dimensions
                .keys()
                .map(|dimension| format!("{}", dimension))
                .collect::<Vec<_>>();
            valid_dimensions.sort();
            let valid_dimensions = valid_dimensions.join(", ");
            let label = format!("not a valid dimension.");
            let msg = format!("{} Options: {}", label, valid_dimensions);

            return Err(LabeledError {
                label,
                msg,
                span: Some(dimension_span),
            });
        };

        let value = call
            .get_flag_value(VALUE_FLAG_NAME)
            .unwrap()
            .as_f64()
            .unwrap();

        let Ok(mut values) = values_function(&unit, value) else {
            let valid_units = units.join(", ");
            let label = format!("not a valid unit.");
            let msg = format!("{} Options: {}", label, valid_units);

            return Err(LabeledError {
                label,
                msg,
                span: Some(unit_span),
            });
        };

        values.sort_by_key(|value| value.0.clone());

        let values: Vec<_> = values
            .iter()
            .map(|(unit, value)| {
                let unit = unit.replace('-', " ");
                let record = Record::from_iter([
                    (UNIT_FLAG_NAME.into(), Value::string(unit, tag)),
                    (VALUE_FLAG_NAME.into(), Value::float(*value, tag)),
                ]);
                Value::record(record, tag)
            })
            .collect();

        Ok(Value::list(values, tag))
    }
}

// TODO: Extract tuple into type?
fn hash_map_tuple<D: Values>(_: D) -> (&'static str, (ValuesFunction, Vec<&'static str>)) {
    (D::name(), (D::values, D::units()))
}
