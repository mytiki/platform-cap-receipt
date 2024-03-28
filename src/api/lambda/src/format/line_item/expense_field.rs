/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use crate::{format::type_class::TypeClass, util::detection::Detection};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpenseField {
    pub page_number: i64,
    #[serde(rename = "Type")]
    pub line_item_expense_field_type: TypeClass,
    pub value_detection: Detection,
}
