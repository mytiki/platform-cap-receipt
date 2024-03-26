/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateResponseLineItemExpenseField {
    product_code: Option<StateResponseSummaryField>,
    item: Option<StateResponseSummaryField>,
    price: Option<StateResponseSummaryField>,
    expense_row: Option<StateResponseSummaryField>,
}