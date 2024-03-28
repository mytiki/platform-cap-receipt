/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Serialize, Deserialize};

use crate::{format::type_class::TypeClass, util::detection::Detection};

use super::group_property::GroupProperty;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SummaryField {
    pub group_properties: Option<Vec<GroupProperty>>,
    pub page_number: i64,
    #[serde(rename = "Type")]
    pub summary_field_type: Option<TypeClass>,
    pub value_detection: Detection,
    pub label_detection: Option<Detection>,
}

