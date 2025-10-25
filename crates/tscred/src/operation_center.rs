use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct OperationCenter {
    #[serde(rename = "Value")]
    pub id: String,

    #[serde(rename = "Text")]
    pub name: String,
}
