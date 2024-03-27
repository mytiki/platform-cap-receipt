/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use super::line_item::LineItem;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    #[serde(rename = "LineItemGroupIndex")]
    pub line_item_group_index: i64,
    pub line_items: Vec<LineItem>,
}