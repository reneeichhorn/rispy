use serde_yaml;
use std::process;
use std::fs::File;
use specs::Configuration;
use runtime::Runtime;
use runtime::state::State;

pub fn create_runtime_from_configuration(path: &str) -> Runtime {
    // Parse the yaml file using the specifications.
    let file = File::open(path).unwrap();
    let config: Configuration = serde_yaml::from_reader(&file)
        .unwrap_or_else(|err| {
            error!("[Parser] Failed parsing program, {}", err);
            process::exit(1);
        });

    debug!("[IO] Configuration file: {:#?}", config);

    Runtime {
        spec: config,
    }
}
