use runtime::value::Value;
use runtime::state::State;
use specs::conditions::Condition;
use specs::conditions::Condition::*;

pub fn evaluate_condition<'a>(condition: &'a Condition, stream: &Value<'a>, state: &State<'a>) -> bool {
    match condition {
        &Gt { ref left, ref right } =>
            Value::from_spec(left).evaluate(stream, state) > Value::from_spec(right).evaluate(stream, state),
        &Eq { ref left, ref right } =>
            Value::from_spec(left).evaluate(stream, state) == Value::from_spec(right).evaluate(stream, state),
        _ => panic!("Failed to evaluate condition: Unimplemented condition type: {:?}", condition),
    }
}
