use specs::links::Link;
use specs::converters::{ Converter, Expression, Value };
use specs::conditions::Condition;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stream {
    pub outputs: Option<Vec<Output>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Output {
    IntoStream { stream: String, links: Option<Vec<Link>>, converter: Option<Converter> },
    IntoStreamOutput { stream: String, converter: Option<Converter> },
    MergeIntoSubstream { object: Option<String>, outputs: Option<Vec<Output>>, converter: Option<Converter> },
    #[serde(rename_all = "camelCase")]
    Condition { condition: Condition, outputs: Option<Vec<Output>>, else_outputs: Option<Vec<Output>> },
    Mutator { expression: Expression, value: Value },
    Ends { outputs: Option<Vec<Output>>, converter: Option<Converter> },
    Capture { outputs: Option<Vec<Output>>, converter: Option<Converter> },
    Breakpoint,
}
