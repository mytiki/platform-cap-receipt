/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SummaryField {
    group_properties: Option<Vec<GroupProperty>>,
    page_number: i64,
    #[serde(rename = "Type")]
    summary_field_type: Option<TypeClass>,
    value_detection: Detection,
    label_detection: Option<Detection>,
}

