use serde::Deserialize;

#[derive(Deserialize)]
pub struct OperationCenter {
    #[serde(rename = "Value")]
    pub id: String,

    #[serde(rename = "Text")]
    pub name: String,
}
