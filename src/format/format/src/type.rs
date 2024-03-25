#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TypeClass {
    confidence: f64,
    text: String,
}