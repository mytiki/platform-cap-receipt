/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;

use crate::util::input_data::InputData;

pub mod block;
pub mod geometry;
pub mod line_item;
pub mod summary;
pub mod type_class;

pub fn process(data: InputData) -> Result<String, Box<Dyn, Error>> {
    let processed_blocks = block::process(data.blocks);

    let processed_line_item_groups = process_line_item_groups(data.line_item_groups);

    let processed_summary_fields = process_summary_fields(data.summary_fields);

    OutputData{
        blocks:  processed_blocks,
        line_item_groups: processed_line_item_groups, 
        summary_fields:  processed_summary_fields
    }
    
}


    

