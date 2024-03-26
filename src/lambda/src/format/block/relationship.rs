/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Relationship {
    ids: Vec<String>,
    #[serde(rename = "Type")]
    relationship_type: TypeEnum,
}