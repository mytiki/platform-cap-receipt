/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use super::super::geometry::geometry::Geometry;
use super::{block_type::BlockType, relationship::Relationship, text_type::TextType};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    pub block_type: BlockType,
    pub geometry: Geometry,
    pub id: String,
    pub relationships: Option<Vec<Relationship>>,
    pub confidence: Option<f64>,
    pub text: Option<String>,
    pub page: Option<i64>,
    pub text_type: Option<TextType>,
}
