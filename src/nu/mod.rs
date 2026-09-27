use crate::values::CATEGORIES;
use nu_plugin::{EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category as NU_CATEGORY, ErrorLabel, Example, LabeledError, Record, Signature, SyntaxShape,
    Value,
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

        let Some((_, conversion_function_map)) =
            CATEGORIES.iter().find(|(name, _)| *name == category)
        else {
            let mut valid_categories: Vec<_> = CATEGORIES.iter().map(|(name, _)| *name).collect();
            valid_categories.sort();
            let valid_categories = valid_categories.join(", ");
            let text = "not a valid category.".to_string();
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

        let conversion_function_map = conversion_function_map();

        let Some(conversion_functions) = conversion_function_map.get(unit.as_str()) else {
            let mut valid_units: Vec<_> = conversion_function_map.keys().copied().collect();
            valid_units.sort();
            let valid_units = valid_units.join(", ");
            let text = "not a valid unit.".to_string();
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

        let mut values: Vec<_> = conversion_functions
            .iter()
            .map(|(unit, conversion_function)| (*unit, conversion_function(value)))
            .collect();
        values.sort_by_key(|(unit, _)| *unit);

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
