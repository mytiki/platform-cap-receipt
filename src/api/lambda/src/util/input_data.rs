/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use crate::format::{block::block::Block, line_item::group::Group, summary::field::SummaryField};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InputData {
    pub document_metadata: DocumentMetadata,
    pub expense_documents: Vec<ExpenseDocument>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentMetadata {
    pages: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpenseDocument {
    pub blocks: Vec<Block>,
    pub expense_index: i64,
    pub line_item_groups: Vec<Group>,
    pub summary_fields: Vec<SummaryField>,
}