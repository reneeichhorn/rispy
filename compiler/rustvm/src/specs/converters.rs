use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Converter {
    Value { value: Value },
    Stream { stream: String },
    Mapper { mapping: HashMap<String, Value> }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Expression {
    Object { name: String, expression: Box<Expression> },
    Value,
    Stream,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Value {
    Stream { values: Vec<Value> },
    Number { value: usize },
    Accessor { expression: Expression },
    Null,
}
