use std::fmt;
use specs::converters;
use specs::converters::Expression;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::Ordering;
use runtime::expression::accessor;
use runtime::state::State;

#[derive(Clone)]
pub enum Value<'a> {
    Null,
    Number(usize),
    Text(String),
    Stream(Rc<RefCell<Fn(Box<FnMut(Value<'a>, bool) + 'a>) + 'a>>),
    Struct(HashMap<String, Value<'a>>),
    Accessor(&'a Expression),
}

impl<'a> Value<'a> {
    pub fn from_spec(spec: &'a converters::Value) -> Value<'a> {
        match spec {
            &converters::Value::Stream { ref values } => {
                Value::Stream(Rc::new(RefCell::new(move | mut emit: Box<FnMut(Value<'a>, bool)> | {
                    for (i, value) in values.iter().enumerate() {
                        emit(Value::from_spec(&value), i >= (values.len() - 1));
                    }
                })))
            },
            &converters::Value::Number { value } => Value::Number(value),
            &converters::Value::Accessor { ref expression } => Value::Accessor(&expression),
            &converters::Value::Null => Value::Null,
            _=> {
                panic!("Failed to create value from spec: Unimplemented value type {:#?}", spec)
            },
        }
    }

    pub fn evaluate(&self, stream: &Value<'a>, state: &State<'a>) -> Value<'a> {
        let evaluation = match self {
            &Value::Accessor(expression) => accessor(expression, stream, state),
            _ => self.clone(),
        };
        debug!("Evaluated {:?} into {:?}", self, evaluation);

        evaluation
    }
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Value) -> bool {
        use self::Value::*;

        match (self, other) {
            (&Null, &Null) => true,
            (&Number(left), &Number(right)) => left == right,
            (&Text(ref left), &Text(ref right)) => left == right,
            _ => false,
        }
    }
}


impl<'a> PartialOrd for Value<'a> {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        use self::Value::*;

        match (self, other) {
            (&Null, &Null) => Some(Ordering::Equal),
            (&Number(left), &Number(right)) => left.partial_cmp(&right),
            (&Text(ref left), &Text(ref right)) => left.partial_cmp(&right),
            _ => None,
        }
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Null => write!(f, "Null"),
            &Value::Number(number) => write!(f, "Number({:?})", number),
            &Value::Text(ref text) => write!(f, "Text({:?})", text),
            &Value::Stream(_) => write!(f, "Stream"),
            &Value::Struct(ref strct) => write!(f, "Struct ({:#?})", strct),
            &Value::Accessor(ref expr) => write!(f, "Accessor ({:#?})", expr),
        }
    }
}
