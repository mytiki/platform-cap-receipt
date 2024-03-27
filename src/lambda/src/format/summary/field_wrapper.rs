/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Serialize, Deserialize};

use super::field::SummaryField;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SummaryFieldsWrapper {
    pub summary_fields: Vec<SummaryField>,
}


