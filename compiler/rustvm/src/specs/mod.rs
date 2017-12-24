pub mod imports;
pub mod streams;
pub mod links;
pub mod converters;
pub mod conditions;
pub mod namespace;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub import_flows: Vec<imports::Import>,
    pub flow_definition: HashMap<String, streams::Stream>,
    pub namespace: HashMap<String, namespace::Namespace>,
}
