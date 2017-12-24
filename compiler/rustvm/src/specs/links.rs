use specs::streams::Output;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub to: String,
    pub outputs: Option<Vec<Output>>,
}
