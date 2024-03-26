/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

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