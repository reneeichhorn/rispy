use specs::converters::Expression;
use specs::converters::Expression::*;
use runtime::value::Value;
use runtime::state::State;

pub fn accessor<'a>(expression: &Expression, stream: &Value<'a>, state: &State<'a>) -> Value<'a> {
    match expression {
        &Object { ref name, ref expression } => object_accessor(&expression, stream, &state.storage[name]).clone(),
        &Stream => stream.clone(),
        _ => panic!("Failed to access data: Expression is not allowed in this context"),
    }
}

pub fn object_accessor<'a>(expression: &Expression, stream: &Value<'a>, object: &Value<'a>) -> Value<'a> {
    match expression {
        &Object { ref name, ref expression } => {
            match object {
                &Value::Struct(ref map) => object_accessor(&expression, stream, &map[name]).clone(),
                _ => panic!("Failed to access data: Type is a non-struct type."),
            }
        },
        &Expression::Value => object.clone(),
        &Stream => stream.clone(),
    }
}

pub fn mutator<'a>(
    expression: &Expression, new_value: &Value<'a>, stream: Value<'a>, state: State<'a>)
        -> (Value<'a>, State<'a>) {

    match expression {
        &Stream => (new_value.evaluate(&stream, &state), state),
        &Object { ref name, ref expression } => {
            let mut new_state = state.clone();
            match new_state.storage.get_mut(name) {
                Some(ref mut value) => {
                    object_mutator(expression, &new_value.evaluate(&stream, &state), value);
                },
                _ => panic!("Failed to mutate data: Type is a non-struct type"),
            };
            (stream, new_state)
        },
        _ => panic!("Failed to mutate data: Expression is not allowed in this context"),
    }
}

pub fn object_mutator<'a>(expression: &Expression, new_value: &Value<'a>, object: &mut Value<'a>) {
    match expression {
        &Object { ref name, ref expression } => {
            match object {
                &mut Value::Struct(ref mut map) => object_mutator(expression, new_value, map.get_mut(name).unwrap_or_else(
                    || panic!(
                        "Failed to mutate data: Internal error#2: Failed to mutably borrow the object: {} in {:#?}", name, expression.clone())
                )),
                _ => panic!("Failed to mutate data: Type is a non-struct type"),
            }
        },
        &Expression::Value => *object = new_value.clone(),
        &Stream => panic!("Failed to mutate data: Mutating internal object in state is not allowed"),
    }
}
