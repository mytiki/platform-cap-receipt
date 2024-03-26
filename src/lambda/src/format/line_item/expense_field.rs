/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItemExpenseField {
    page_number: i64,
    #[serde(rename = "Type")]
    line_item_expense_field_type: TypeClass,
    value_detection: Detection,
}