/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutputData {
    blocks: Vec<ResponseBlock>,
    line_item_groups: Vec<StateResponseLineItemGroup>,
    summary_fields: Vec<HashMap<String, StateResponseSummaryField>>,
}
