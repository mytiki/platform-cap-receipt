/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use super::expense_field::ExpenseField;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItem {
    pub line_item_expense_fields: Vec<ExpenseField>,
}
