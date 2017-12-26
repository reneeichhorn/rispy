use runtime::value::Value;
use bindings::ExternalStream;

pub struct StdOut;

impl ExternalStream for StdOut {
    fn handle_emit(&self, value: Value, end: bool) {
        debug!("Printing value to stdout: {:#?}", value);

        match value {
            Value::Number(number) => print!("{}", number),
            _ => error!("Tried to print unsupported value type {:#?}", value),
        }
    }
}
