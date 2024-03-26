/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod block;

use serde::{self, Serialize, Deserialize};
use std::collections::HashMap;


fn format(data: InputData) {
    let processed_blocks = block::process(data.blocks);

    let processed_line_item_groups = process_line_item_groups(data.line_item_groups);

    let processed_summary_fields = process_summary_fields(data.summary_fields);

    OutputData{
        blocks:  processed_blocks,
        line_item_groups: processed_line_item_groups, 
        summary_fields:  processed_summary_fields
    }
    
}


    

