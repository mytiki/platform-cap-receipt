/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateResponseSummaryField {
    confidence_key: f64,
    confidence_value: f64,
    value: String,
}