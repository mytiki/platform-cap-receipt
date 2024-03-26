/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
pub enum BlockType {
    #[serde(rename = "LINE")]
    Line,
    #[serde(rename = "PAGE")]
    Page,
    #[serde(rename = "WORD")]
    Word,
}