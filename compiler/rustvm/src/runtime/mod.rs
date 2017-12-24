use std::process;
use specs::Configuration;
use specs::streams::Output;
use specs::streams::Output::*;
use runtime::value::Value;
use runtime::state::State;
use runtime::conditions::evaluate_condition;
use runtime::expression::{ mutator };

pub mod converter;
pub mod value;
pub mod state;
pub mod conditions;
pub mod expression;

pub struct Runtime {
    pub spec: Configuration,
}

impl Runtime {
    pub fn resolve(&self, name: String) -> String {
        for import in &self.spec.import_flows {
            if import.name == name {
                return import.from.clone();
            }
        }
        return name;
    }

    pub fn reverse_resolve(&self, name: String) -> String {
        for import in &self.spec.import_flows {
            if import.from == name {
                return import.name.clone();
            }
        }
        return name;
    }

    pub fn execute(&self) {
        debug!("Executing runtime...");

        // Find application start
        let entry = self.reverse_resolve(String::from("core/application-start"));
        self.execute_stream_with(State::new(), entry, Value::Number(1), true);
    }

    pub fn execute_stream_with<'a>(&'a self, state: State<'a>, stream: String, value: Value<'a>, end: bool) {
        // Get stream specification
        let stream_spec = self.spec.flow_definition.get(&stream).unwrap_or_else(|| {
            error!("Can not execute invalid stream: {}", stream);
            process::exit(1);
        });

        debug!("Executing stream {} with {:?} [isEnd:{}]", stream, value, end);

        self.execute_spec_with(state, &stream_spec.outputs, value, end);
    }

    pub fn execute_spec_with<'a>(&'a self, mut state: State<'a>, outputs: &'a Option<Vec<Output>>, mut value: Value<'a>, end: bool) {
        debug!("State: {:#?}", state);
        match outputs {
            &Some(ref outputs) => {
                for output in outputs {
                    let mut new_state = state.clone();
                    match output {
                        &IntoStream { ref stream, ref links, ref converter } => {
                            // Simply convert and put into new stream.
                            let new_value = match converter {
                                &Some(ref converter) => converter::convert_value(&value, &converter),
                                &None => value.clone(),
                            };
                            self.execute_stream_with(new_state, self.resolve(stream.clone()), new_value, end);
                        },
                        &MergeIntoSubstream { ref object, ref outputs, converter: _ } => {
                            // Initialize the state object if given
                            match object {
                                &Some(ref namespace) => new_state.initialize_from_spec(&namespace, &self.spec.namespace[namespace]),
                                &None => {},
                            };

                            // Match the stream and merge emitted values into target streams
                            match value {
                                Value::Stream(ref mut emitter) => {
                                    emitter.borrow_mut()(Box::new(move | inner_value, inner_end | {
                                        debug!("Merging into substream: {:?} {}", inner_value, inner_end);
                                        self.execute_spec_with(new_state.clone(), &outputs, inner_value, inner_end);
                                    }));
                                },
                                _ => panic!("Failed to merge into substream: Incoming value was not a stream"),
                            }
                        },
                        &Condition { ref condition, ref outputs, ref else_outputs } => {
                            if evaluate_condition(condition, &value, &new_state) {
                                debug!("Condition was evaluated as true");
                                self.execute_spec_with(new_state, &outputs, value.clone(), end);
                            } else {
                                debug!("Condition was evaluated as false");
                                self.execute_spec_with(new_state, &else_outputs, value.clone(), end);
                            }
                        },
                        &Mutator { ref expression, value: ref new_value } => {
                            let (new_stream_value, new_mutated_state) =
                                mutator(&expression, &Value::from_spec(&new_value), value.clone(), new_state.clone());

                            debug!("Mutator mutated from {:?} to {:?}",
                                (value.clone(), new_state.clone()),
                                (new_stream_value, new_mutated_state.clone()));

                            state = new_mutated_state;
                        },
                        _ => {
                            error!("Failed to execute output: Unimplemented operation {:?}", output)
                        },
                    }
                }
            },
            &None => {
                debug!("Stream was empty, continue..");
                return;
            }
        }
    }
}
