fn process_blocks(blocks: &Vec<Block>) -> Vec<StateResponseBlock> {
  blocks.iter().map(|block| {
      StateResponseBlock{
          confidence: block.confidence,
          text: block.text.clone(),
      }
  }).collect()
} 

#[derive(Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "CHILD")]
    Child,
}



#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseBlock {
    confidence: f64,
    text: String,
}


#[derive(Serialize, Deserialize)]
pub enum TextType {
    #[serde(rename = "PRINTED")]
    Printed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    block_type: BlockType,
    geometry: Geometry,
    id: String,
    relationships: Vec<Relationship>,
    confidence: f64,
    text: String,
    page: Option<i64>,
    text_type: Option<TextType>,
}

#[derive(Serialize, Deserialize)]
pub enum BlockType {
    #[serde(rename = "LINE")]
    Line,
    #[serde(rename = "PAGE")]
    Page,
    #[serde(rename = "WORD")]
    Word,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Relationship {
    ids: Vec<String>,
    #[serde(rename = "Type")]
    relationship_type: TypeEnum,
}
