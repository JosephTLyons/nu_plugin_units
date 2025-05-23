use std::collections::HashMap;

use crate::values::*;
use nu_plugin::{EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category as NU_CATEGORY, ErrorLabel, Example, LabeledError, Record, Signature, SyntaxShape,
    Value,
};

pub struct Units;

const CATEGORY_FLAG_NAME: &'static str = "category";
const UNIT_FLAG_NAME: &'static str = "unit";
const VALUE_FLAG_NAME: &'static str = "value";

impl Plugin for Units {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Units)]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}

impl SimplePluginCommand for Units {
    type Plugin = Units;

    fn name(&self) -> &str {
        "units"
    }

    fn description(&self) -> &str {
        "Convert between units"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .required_named(
                CATEGORY_FLAG_NAME,
                SyntaxShape::String,
                "specify the category",
                Some('c'),
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
            .category(NU_CATEGORY::Generators)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "units -c time -u years -v 1".into(),
            example: "Display various units of time equivalent to 1 year".into(),
            result: None,
        }]
    }

    fn run(
        &self,
        _: &Self::Plugin,
        _: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _: &Value,
    ) -> Result<Value, LabeledError> {
        let tag = call.head;

        // The `unwrap()`s are safe, since the flags, arguments, and arguments types are enforced by the signature
        // The `unwrap()`s here are to make sure the strings looked up matches the ones in the signature
        let category = call.get_flag_value(CATEGORY_FLAG_NAME).unwrap();
        let category_span = category.span();
        let category = category.into_string().unwrap();

        let unit = call.get_flag_value(UNIT_FLAG_NAME).unwrap();
        let unit_span = unit.span();
        let unit = unit.into_string().unwrap();

        let categories: HashMap<_, _> = HashMap::from_iter([
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

        let Some((values_function, units)) = categories.get(category.as_str()) else {
            let mut valid_categories = categories
                .keys()
                .map(|category| format!("{}", category))
                .collect::<Vec<_>>();
            valid_categories.sort();
            let valid_categories = valid_categories.join(", ");
            let text = format!("not a valid category.");
            let msg = format!("{} Options: {}", text, valid_categories);

            return Err(LabeledError {
                msg,
                labels: Box::new(vec![ErrorLabel {
                    text,
                    span: category_span,
                }]),
                code: None,
                url: None,
                help: None,
                inner: Box::new(vec![]),
            });
        };

        let value = call
            .get_flag_value(VALUE_FLAG_NAME)
            .unwrap()
            .as_float()
            .unwrap();

        let Ok(mut values) = values_function(&unit, value) else {
            let valid_units = units.join(", ");
            let text = format!("not a valid unit.");
            let msg = format!("{} Options: {}", text, valid_units);

            return Err(LabeledError {
                msg,
                labels: Box::new(vec![ErrorLabel {
                    text,
                    span: unit_span,
                }]),
                code: None,
                url: None,
                help: None,
                inner: Box::new(vec![]),
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
fn hash_map_tuple<D: Category>(_: D) -> (&'static str, (ValuesFunction, Vec<&'static str>)) {
    (D::name(), (D::values, D::units()))
}
