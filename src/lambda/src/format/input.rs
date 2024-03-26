/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InputData{
    blocks: Vec<Block>, 
    line_item_groups: Vec<LineItemGroup>, 
    summary_fields: [SummaryField]
}




