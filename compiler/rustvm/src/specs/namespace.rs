use specs::converters::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Namespace {
    #[serde(rename_all = "camelCase")]
    Object { initial_value: HashMap<String, Value> },
}
