use specs::converters::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Condition {
    Gt { left: Value, right: Value },
    Lt { left: Value, right: Value },
    Eq { left: Value, right: Value },
}
