/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use super::type_enum::TypeEnum;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Relationship {
    ids: Vec<String>,
    #[serde(rename = "Type")]
    relationship_type: TypeEnum,
}