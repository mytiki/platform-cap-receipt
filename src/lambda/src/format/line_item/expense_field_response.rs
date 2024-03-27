/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use crate::util::field_response::FieldResponse;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ExpenseFieldResponse {
    pub product_code: Option<FieldResponse>,
    pub item: Option<FieldResponse>,
    pub price: Option<FieldResponse>,
    pub expense_row: Option<FieldResponse>,
}
