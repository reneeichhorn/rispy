use std::collections::HashMap;
use runtime::value::Value;
use specs::namespace::Namespace;

#[derive(Debug, Clone)]
pub struct State<'a> {
    pub storage: HashMap<String, Value<'a>>,
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        State {
            storage: HashMap::new(),
        }
    }

    pub fn initialize_from_spec(&mut self, name: &String, spec: &'a Namespace) {
        match spec {
            &Namespace::Object { ref initial_value } => {
                let mut values = HashMap::new();
                for (name, spec_value) in initial_value {
                    values.insert(name.clone(), Value::<'a>::from_spec(&spec_value));
                }

                self.storage.insert(name.clone(), Value::Struct(values));
            },
            _ => panic!("Could not initialize object: Unimplemented namespace type: {:?}"),
        }
    }
}
