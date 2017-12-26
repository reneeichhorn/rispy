#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::rc::Rc;

mod specs;
mod parser;
mod runtime;
mod bindings;

fn main() {
    // Initialize logging
    pretty_env_logger::init().unwrap();
    debug!("Debug enabled!");

    // Initialize runtime
    let mut runtime = parser::create_runtime_from_configuration("../../examples/sort/program.yaml");
    let mut state = runtime::state::State::new();

    // Load std bindings
    runtime.add_external_stream(&String::from("core/terminal/stdout"), Box::new(bindings::terminal::StdOut));

    runtime.execute(&mut state);
}
