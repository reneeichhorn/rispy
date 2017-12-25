use std::process;
use specs::Configuration;
use specs::streams::Output;
use specs::streams::Output::*;
use runtime::value::Value;
use runtime::state::State;
use runtime::conditions::evaluate_condition;
use runtime::expression::{ mutator };
use runtime::links::LinkWatcher;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub mod converter;
pub mod value;
pub mod state;
pub mod conditions;
pub mod expression;
pub mod links;
pub mod capturing;

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

    pub fn execute<'a>(&'a self, state: &mut State<'a>) {
        debug!("Executing runtime...");

        // Find application start
        let entry = self.reverse_resolve(String::from("core/application-start"));
        self.execute_stream_with(state, entry, Value::Number(1), true, LinkWatcher::new());
    }

    pub fn execute_stream_with<'a>(&'a self, state: &mut State<'a>, stream: String, value: Value<'a>, end: bool, watcher: LinkWatcher<'a>) {
        debug!("Executing stream {} with {:?} [isEnd:{}]", stream, value, end);

        // Get stream specification
        let stream_spec = self.spec.flow_definition.get(&stream).unwrap_or_else(|| {
            error!("Can not execute invalid stream: {}", stream);
            process::exit(1);
        });

        self.execute_spec_with(state, &stream_spec.outputs, value, end, watcher, format!("stream.{}", stream));
    }

    pub fn execute_spec_with<'a>(
        &'a self,
        state: &mut State<'a>,
        outputs: &'a Option<Vec<Output>>,
        mut value: Value<'a>,
        end: bool,
        mut watcher: LinkWatcher<'a>,
        uuid: String,
    ) {
        debug!("Executing {}", uuid.clone());
        debug!("State: {:#?}", state);
        match outputs {
            &Some(ref outputs) => {
                for (outputIndex, output) in outputs.iter().enumerate() {
                    match output {
                        &IntoStream { ref stream, ref links, ref converter } => {
                            // Simply convert and put into new stream.
                            let mut new_value = match converter {
                                &Some(ref converter) => converter::convert_value(&value, &converter),
                                &None => value.clone(),
                            };

                            // Add notifiers for links
                            if links.is_some() {
                                match links {
                                    &Some(ref links) => {
                                        for (linkIndex, link) in links.iter().enumerate() {
                                            let mut cloned = state.clone();
                                            let cloned_watcher = watcher.clone();
                                            let mut cloned_uuid = uuid.clone();

                                            watcher.add_watcher(link.clone().to, Rc::new(RefCell::new(move | inner_value, inner_end | {
                                                debug!("Link resolved to '{:}': {:?} {}", link.to, inner_value, inner_end);
                                                self.execute_spec_with(
                                                    &mut cloned,
                                                    &link.outputs,
                                                    inner_value,
                                                    inner_end,
                                                    cloned_watcher.clone(),
                                                    format!("{}.{}.link.{}", cloned_uuid.clone(), outputIndex, linkIndex),
                                                );
                                            })));
                                        }
                                    },
                                    &None => {},
                                }
                            }

                            self.execute_stream_with(state, self.resolve(stream.clone()), new_value, end, watcher.clone());
                        },
                        &MergeIntoSubstream { ref object, ref outputs, converter: _ } => {
                            // Initialize the state object if given
                            match object {
                                &Some(ref namespace) => state.initialize_from_spec(&namespace, &self.spec.namespace[namespace]),
                                &None => {},
                            };

                            // Match the stream and merge emitted values into target streams
                            match value {
                                Value::Stream(ref mut emitter) => {
                                    let mut cloned = state.clone();
                                    let mut cloned_watcher = watcher.clone();
                                    let mut cloned_uuid = uuid.clone();

                                    emitter.borrow_mut()(Box::new(move | inner_value, inner_end | {
                                        debug!("Merging into substream: {:?} {}", inner_value, inner_end);
                                        //self.execute_spec_with(&mut state, &outputs, inner_value, inner_end);
                                        self.execute_spec_with(
                                            &mut cloned,
                                            &outputs,
                                            inner_value,
                                            inner_end,
                                            cloned_watcher.clone(),
                                            format!("{}.{}.substream", cloned_uuid.clone(), outputIndex),
                                        );
                                    }));
                                },
                                _ => panic!("Failed to merge into substream: Incoming value was not a stream"),
                            }
                        },
                        &Condition { ref condition, ref outputs, ref else_outputs } => {
                            if evaluate_condition(condition, &value, &state) {
                                debug!("Condition was evaluated as true");
                                self.execute_spec_with(
                                    state,
                                    &outputs,
                                    value.clone(),
                                    end,
                                    watcher.clone(),
                                    format!("{}.{}.condition.true", uuid.clone(), outputIndex),
                                );
                            } else {
                                debug!("Condition was evaluated as false");
                                self.execute_spec_with(
                                    state,
                                    &else_outputs,
                                    value.clone(),
                                    end,
                                    watcher.clone(),
                                    format!("{}.{}.condition.false", uuid.clone(), outputIndex),
                                );
                            }
                        },
                        &Mutator { ref expression, value: ref new_value } => {
                            debug!("Mutating data from ({:#?} {:#?})", value.clone(), state.clone());
                            mutator(&expression, &Value::from_spec(&new_value), &mut value, state);
                            debug!("\t to ({:#?} {:#?})", value.clone(), state.clone());
                        },
                        &Ends { ref outputs, ref converter } => {
                            if end {
                                debug!("End was reached with {:?}", value);
                                self.execute_spec_with(
                                    state,
                                    &outputs,
                                    value.clone(),
                                    end,
                                    watcher.clone(),
                                    format!("{}.{}.ends", uuid.clone(), outputIndex),
                                );
                            }
                        },
                        &IntoStreamOutput { ref stream, ref converter } => {
                            debug!("Streaming into output of '{:}' : {:#?} {}", stream, value, end);

                            // Simply convert and put into stream output.
                            let mut new_value = match converter {
                                &Some(ref converter) => converter::convert_value(&value, &converter),
                                &None => value.clone(),
                            };

                            watcher.resolve_links(stream.clone(), &new_value, end);
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
