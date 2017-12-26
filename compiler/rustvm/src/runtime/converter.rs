use specs::converters::Converter;
use runtime::value::Value;

pub fn convert_value<'a>(value: &Value<'a>, converter: &'a Converter) -> Value<'a> {
    match converter {
        &Converter::Value { value: ref spec_value } => {
            Value::from_spec(&spec_value)
        },
        _ => {
            error!("Failed to convert: Unimplemented converter was used: {:#?}", converter);
            value.clone()
        }
    }
}
