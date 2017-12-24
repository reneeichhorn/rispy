#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub name: String,
    pub from: String,
}
