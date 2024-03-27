/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Serialize, Deserialize};

use super::expense_field_response::ExpenseFieldResponse;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItemResponse {
    pub line_item_expense_fields: Vec<ExpenseFieldResponse>,
}
