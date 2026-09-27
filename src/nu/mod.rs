use crate::values::{convert, ConversionError};
use nu_plugin::{EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category as NU_CATEGORY, Example, LabeledError, Record, Signature, SyntaxShape, Value,
};

pub struct Units;

const CATEGORY_FLAG_NAME: &str = "category";
const UNIT_FLAG_NAME: &str = "unit";
const VALUE_FLAG_NAME: &str = "value";

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

    fn examples(&self) -> Vec<Example<'_>> {
        vec![Example {
            description: "Display various units of time equivalent to 1 year",
            example: "units -c time -u years -v 1",
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

        let value = call
            .get_flag_value(VALUE_FLAG_NAME)
            .unwrap()
            .as_float()
            .unwrap();

        let values = convert(&category, &unit, value).map_err(|error| {
            let (text, options, span) = match error {
                ConversionError::UnknownCategory { valid_categories } => {
                    ("not a valid category.", valid_categories, category_span)
                }
                ConversionError::UnknownUnit { valid_units } => {
                    ("not a valid unit.", valid_units, unit_span)
                }
            };
            let options = options.join(", ");
            LabeledError::new(format!("{text} Options: {options}")).with_label(text, span)
        })?;

        let values: Vec<_> = values
            .into_iter()
            .map(|(unit, value)| {
                let unit = unit.replace('-', " ");
                let record = Record::from_iter([
                    (UNIT_FLAG_NAME.into(), Value::string(unit, tag)),
                    (VALUE_FLAG_NAME.into(), Value::float(value, tag)),
                ]);
                Value::record(record, tag)
            })
            .collect();

        Ok(Value::list(values, tag))
    }
}
