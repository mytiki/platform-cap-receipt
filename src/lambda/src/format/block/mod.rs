/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

mod block;
mod response_block;
mod block_type;
mod relationship;
mod text_type;
mod type_enum;
use block::Block;
use response_block::ResponseBlock;

pub fn process(blocks: &Vec<Block>) -> Vec<ResponseBlock> {
    blocks
        .iter()
        .map(|block| ResponseBlock {
            confidence: block.confidence,
            text: block.text.clone(),
        })
        .collect()
}

#[test]
fn test_process_blocks_json() {
    let json_str = r#"{
        "BlockType": "LINE",
        "Confidence": 99.76323,
        "Geometry": {
            "BoundingBox": {
                "Height": 0.014501625,
                "Left": 0.26956508,
                "Top": 0.11528842,
                "Width": 0.12523548
            },
            "Polygon": [
                {
                    "X": 0.269567,
                    "Y": 0.11630608
                },
                {
                    "X": 0.39480057,
                    "Y": 0.11528842
                },
                {
                    "X": 0.39477816,
                    "Y": 0.12879479
                },
                {
                    "X": 0.26956508,
                    "Y": 0.12979004
                }
            ]
        },
        "Id": "04df9ccd-95c6-462e-b344-64f4640f82a1",
        "Relationships": [
            {
                "Ids": [
                    "18eb9cab-3cc1-4f57-96d8-03428b1c5d0f",
                    "5bed20e8-6750-4361-8bb5-7d0371708a7e",
                    "f838c320-a2f6-42ea-9641-80d03cf1f9dc"
                ],
                "Type": "CHILD"
            }
        ],
        "Text": "to win $1000"
    }"#;

    let block: Block = serde_json::from_str(json_str).unwrap();

    let result = process(&vec![block]);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].confidence, 99.76323);
    assert_eq!(result[0].text, "to win $1000");
}
