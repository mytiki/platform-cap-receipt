/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::format::{block::{self, response_block::ResponseBlock}, line_item::group_response::GroupResponse};

use super::field_response::FieldResponse;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutputData {
    pub blocks: Vec<ResponseBlock>,
    pub line_item_groups: Vec<GroupResponse>,
    pub summary_fields: Vec<HashMap<String, FieldResponse>>,
}
