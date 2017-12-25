#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod specs;
mod parser;
mod runtime;

fn main() {
    pretty_env_logger::init().unwrap();
    debug!("Debug enabled!");

    let runtime = parser::create_runtime_from_configuration("../../examples/sort/program.yaml");
    let mut state = runtime::state::State::new();
    runtime.execute(&mut state);
}
