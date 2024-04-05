/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;


use crate::util::{input_data::InputData, output_data::OutputData};

pub mod block;
pub mod geometry;
pub mod line_item;
pub mod summary;
pub mod type_class;

pub fn process(data: InputData) -> Result<OutputData, Box<dyn Error>> {
    let processed_blocks = block::process(&data.expense_documents[0].blocks);

    let processed_line_item_groups = line_item::process(&data.expense_documents[0].line_item_groups);

    let processed_summary_fields = summary::process(&data.expense_documents[0].summary_fields);

    let result = OutputData{
      blocks:  processed_blocks,
      line_item_groups: processed_line_item_groups, 
      summary_fields:  processed_summary_fields
    };
    let json_string = result;
    
    Ok(json_string)
} 

