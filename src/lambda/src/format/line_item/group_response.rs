/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseLineItemGroup {
    line_item_group_index: i64,
    line_items: Vec<StateResponseLineItem>,
}