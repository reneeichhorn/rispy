use runtime::Runtime;
use runtime::value::Value;

pub mod string;
pub mod terminal;

pub trait ExternalStream {
    fn handle_emit(&self, value: Value, end: bool);
}
