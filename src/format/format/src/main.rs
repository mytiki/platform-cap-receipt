use serde::{self, Serialize, Deserialize};
use std::collections::HashMap;

fn function_handler(data: InputData) {
    let processed_blocks = process_blocks(data.blocks);

    let processed_line_item_groups = process_line_item_groups(data.line_item_groups);

    let processed_summary_fields = process_summary_fields(data.summary_fields);

    OutputData{
        blocks:  processed_blocks,
        line_item_groups: processed_line_item_groups, 
        summary_fields:  processed_summary_fields
    }
    
}

fn main(data: InputData){
    function_handler(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_blocks() {
        let block = Block {
            page: Some(1),
            text_type: Some(TextType::Printed),
            block_type: BlockType::Line,
            confidence: 99.76323,
            geometry:{ Geometry {
                bounding_box: {BoundingBox {
                    height: 0.014501625,
                    left: 0.26956508,
                    top: 0.11528842,
                    width: 0.12523548
                }},
                polygon: vec![
                    {Polygon {
                        x: 0.269567,
                        y: 0.11630608
                    }},
                    {Polygon {
                        x: 0.269567,
                        y: 0.11630608
                    }},
                    {Polygon {
                        x: 0.269567,
                        y: 0.11630608
                    }},
                    {Polygon {
                        x: 0.269567,
                        y: 0.11630608
                    }}
                ]
            }},
            id: String::from("04df9ccd-95c6-462e-b344-64f4640f82a1"),
            relationships: vec![
                {Relationship {
                    ids: vec![
                        String::from("18eb9cab-3cc1-4f57-96d8-03428b1c5d0f"),
                        String::from("18eb9cab-3cc1-4f57-96d8-03428b1c5d0f"),
                        String::from("18eb9cab-3cc1-4f57-96d8-03428b1c5d0f"),
                    ],
                    relationship_type: TypeEnum::Child
                }}
            ],
            text: String::from("to win $1000")
        };


        let result = process_blocks(&vec![block]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].confidence, 99.76323);
        assert_eq!(result[0].text, "to win $1000");
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

        let result = process_blocks(&vec![block]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].confidence, 99.76323);
        assert_eq!(result[0].text, "to win $1000");
    }

    #[test]
    fn test_process_line_items() {
        let json_str = r#" {
                "LineItemGroupIndex": 1,
                "LineItems": [
                    {
                        "LineItemExpenseFields": [
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 92.48912,
                                    "Text": "PRODUCT_CODE"
                                },
                                "ValueDetection": {
                                    "Confidence": 92.48831,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.014868261,
                                            "Left": 0.40393516,
                                            "Top": 0.37031016,
                                            "Width": 0.12765871
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.4039605,
                                                "Y": 0.37091812
                                            },
                                            {
                                                "X": 0.53159386,
                                                "Y": 0.37031016
                                            },
                                            {
                                                "X": 0.53154635,
                                                "Y": 0.38459447
                                            },
                                            {
                                                "X": 0.40393516,
                                                "Y": 0.38517842
                                            }
                                        ]
                                    },
                                    "Text": "020108870398"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.993645,
                                    "Text": "ITEM"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.98706,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.014433219,
                                            "Left": 0.26894325,
                                            "Top": 0.37184215,
                                            "Width": 0.12586512
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.26894513,
                                                "Y": 0.37244022
                                            },
                                            {
                                                "X": 0.39480835,
                                                "Y": 0.37184215
                                            },
                                            {
                                                "X": 0.39478528,
                                                "Y": 0.38570032
                                            },
                                            {
                                                "X": 0.26894325,
                                                "Y": 0.38627535
                                            }
                                        ]
                                    },
                                    "Text": "6 WING PLATE"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99941,
                                    "Text": "PRICE"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.98682,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.013815278,
                                            "Left": 0.6035521,
                                            "Top": 0.36976326,
                                            "Width": 0.04285302
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.60360926,
                                                "Y": 0.3699671
                                            },
                                            {
                                                "X": 0.6464051,
                                                "Y": 0.36976326
                                            },
                                            {
                                                "X": 0.64634085,
                                                "Y": 0.38338235
                                            },
                                            {
                                                "X": 0.6035521,
                                                "Y": 0.38357854
                                            }
                                        ]
                                    },
                                    "Text": "3.98"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99553,
                                    "Text": "EXPENSE_ROW"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.988434,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.016612941,
                                            "Left": 0.26894325,
                                            "Top": 0.3696624,
                                            "Width": 0.39863235
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.26894525,
                                                "Y": 0.37156126
                                            },
                                            {
                                                "X": 0.6675756,
                                                "Y": 0.3696624
                                            },
                                            {
                                                "X": 0.667502,
                                                "Y": 0.3844541
                                            },
                                            {
                                                "X": 0.26894325,
                                                "Y": 0.38627535
                                            }
                                        ]
                                    },
                                    "Text": "6 WING PLATE 020108870398 3.98 P"
                                }
                            }
                        ]
                    },
                    {
                        "LineItemExpenseFields": [
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 91.53571,
                                    "Text": "PRODUCT_CODE"
                                },
                                "ValueDetection": {
                                    "Confidence": 91.53119,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.014457082,
                                            "Left": 0.40390816,
                                            "Top": 0.3859109,
                                            "Width": 0.12733753
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.4039328,
                                                "Y": 0.38649127
                                            },
                                            {
                                                "X": 0.5312457,
                                                "Y": 0.3859109
                                            },
                                            {
                                                "X": 0.5311995,
                                                "Y": 0.3998109
                                            },
                                            {
                                                "X": 0.40390816,
                                                "Y": 0.40036798
                                            }
                                        ]
                                    },
                                    "Text": "063099656595"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99864,
                                    "Text": "ITEM"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.995415,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.013725392,
                                            "Left": 0.26964083,
                                            "Top": 0.3874782,
                                            "Width": 0.07323323
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.26964277,
                                                "Y": 0.38781136
                                            },
                                            {
                                                "X": 0.34287405,
                                                "Y": 0.3874782
                                            },
                                            {
                                                "X": 0.3428602,
                                                "Y": 0.4008834
                                            },
                                            {
                                                "X": 0.26964083,
                                                "Y": 0.4012036
                                            }
                                        ]
                                    },
                                    "Text": "ASST 27"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99483,
                                    "Text": "PRICE"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.99137,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.014598562,
                                            "Left": 0.60273224,
                                            "Top": 0.38440824,
                                            "Width": 0.04398374
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.60279256,
                                                "Y": 0.38460904
                                            },
                                            {
                                                "X": 0.64671594,
                                                "Y": 0.38440824
                                            },
                                            {
                                                "X": 0.64664793,
                                                "Y": 0.39881432
                                            },
                                            {
                                                "X": 0.60273224,
                                                "Y": 0.39900678
                                            }
                                        ]
                                    },
                                    "Text": "4.88"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99759,
                                    "Text": "EXPENSE_ROW"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.992935,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.016893178,
                                            "Left": 0.2696315,
                                            "Top": 0.38431045,
                                            "Width": 0.3984733
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.26963368,
                                                "Y": 0.38613203
                                            },
                                            {
                                                "X": 0.66810477,
                                                "Y": 0.38431045
                                            },
                                            {
                                                "X": 0.6680293,
                                                "Y": 0.39946148
                                            },
                                            {
                                                "X": 0.2696315,
                                                "Y": 0.40120363
                                            }
                                        ]
                                    },
                                    "Text": "ASST 27 063099656595 4.88 X"
                                }
                            }
                        ]
                    },
                    {
                        "LineItemExpenseFields": [
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 92.529335,
                                    "Text": "PRODUCT_CODE"
                                },
                                "ValueDetection": {
                                    "Confidence": 92.52755,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.01439942,
                                            "Left": 0.40388057,
                                            "Top": 0.40150425,
                                            "Width": 0.12732634
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.40390518,
                                                "Y": 0.4020585
                                            },
                                            {
                                                "X": 0.5312069,
                                                "Y": 0.40150425
                                            },
                                            {
                                                "X": 0.53116083,
                                                "Y": 0.41537267
                                            },
                                            {
                                                "X": 0.40388057,
                                                "Y": 0.41590366
                                            }
                                        ]
                                    },
                                    "Text": "063099656644"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99924,
                                    "Text": "ITEM"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.998184,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.013773425,
                                            "Left": 0.26893908,
                                            "Top": 0.40285733,
                                            "Width": 0.09497223
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.2689409,
                                                "Y": 0.40327007
                                            },
                                            {
                                                "X": 0.3639113,
                                                "Y": 0.40285733
                                            },
                                            {
                                                "X": 0.36389405,
                                                "Y": 0.41623485
                                            },
                                            {
                                                "X": 0.26893908,
                                                "Y": 0.41663077
                                            }
                                        ]
                                    },
                                    "Text": "CUTIE CAR"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99979,
                                    "Text": "PRICE"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.99737,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.014743146,
                                            "Left": 0.59436285,
                                            "Top": 0.4000264,
                                            "Width": 0.05213054
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.5944222,
                                                "Y": 0.4002538
                                            },
                                            {
                                                "X": 0.64649343,
                                                "Y": 0.4000264
                                            },
                                            {
                                                "X": 0.64642483,
                                                "Y": 0.41455212
                                            },
                                            {
                                                "X": 0.59436285,
                                                "Y": 0.41476956
                                            }
                                        ]
                                    },
                                    "Text": "12.88"
                                }
                            },
                            {
                                "PageNumber": 1,
                                "Type": {
                                    "Confidence": 99.99953,
                                    "Text": "EXPENSE_ROW"
                                },
                                "ValueDetection": {
                                    "Confidence": 99.9951,
                                    "Geometry": {
                                        "BoundingBox": {
                                            "Height": 0.016699102,
                                            "Left": 0.26893908,
                                            "Top": 0.39993167,
                                            "Width": 0.3992505
                                        },
                                        "Polygon": [
                                            {
                                                "X": 0.26894113,
                                                "Y": 0.40167508
                                            },
                                            {
                                                "X": 0.6681896,
                                                "Y": 0.39993167
                                            },
                                            {
                                                "X": 0.6681146,
                                                "Y": 0.41496634
                                            },
                                            {
                                                "X": 0.26893908,
                                                "Y": 0.41663077
                                            }
                                        ]
                                    },
                                    "Text": "CUTIE CAR 063099656644 12.88 X"
                                }
                            }
                        ]
                    }
                ]
            }
        "#;

        let line_items: LineItemGroup = serde_json::from_str(json_str).unwrap();

        let result = process_line_item_groups(&vec![line_items]);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0].line_item_group_index, 1);

        assert_eq!(result[0].line_items.len(), 3);

        let line_item_expense_fields = &result[0].line_items[0].line_item_expense_fields;
        assert_eq!(line_item_expense_fields.len(), 4); 

        for field in line_item_expense_fields {
            match field.product_code {
                Some(ref summary_field) => {
                    assert_eq!(summary_field.value, "020108870398");
                }
                None => {}
            }
            match field.item {
                Some(ref summary_field) => {
                    assert_eq!(summary_field.value, "6 WING PLATE");
                }
                None => {}
            }
            match field.price {
                Some(ref summary_field) => {
                    assert_eq!(summary_field.value, "3.98");
                }
                None => {}
            }
            match field.expense_row {
                Some(ref summary_field) => {
                    assert_eq!(
                        summary_field.value,
                        "6 WING PLATE 020108870398 3.98 P"
                    );
                }
                None => {}
            }
        }
    }

    #[test]
    fn test_process_summary(){
        let json_str = r#"
        {
            "SummaryFields": [{
                    "GroupProperties": [
                        {
                            "Id": "9c053dc8-7cc1-4232-a8a4-04a188b0d047",
                            "Types": [
                                "VENDOR"
                            ]
                        }
                    ],
                    "PageNumber": 1,
                    "Type": {
                        "Confidence": 99.900925,
                        "Text": "ADDRESS"
                    },
                    "ValueDetection": {
                        "Confidence": 99.889496,
                        "Geometry": {
                            "BoundingBox": {
                                "Height": 0.030612579,
                                "Left": 0.36246267,
                                "Top": 0.32429105,
                                "Width": 0.19942787
                            },
                            "Polygon": [
                                {
                                    "X": 0.36250025,
                                    "Y": 0.32536086
                                },
                                {
                                    "X": 0.56189054,
                                    "Y": 0.32429105
                                },
                                {
                                    "X": 0.5617812,
                                    "Y": 0.35391167
                                },
                                {
                                    "X": 0.36246267,
                                    "Y": 0.3549036
                                }
                            ]
                        },
                        "Text": "2717 ROCK ISLAND PL\nBISMARCK ND 58504"
                    }
                },
                {
                    "GroupProperties": [
                        {
                            "Id": "9c053dc8-7cc1-4232-a8a4-04a188b0d047",
                            "Types": [
                                "VENDOR"
                            ]
                        }
                    ],
                    "PageNumber": 1,
                    "Type": {
                        "Confidence": 99.900925,
                        "Text": "STREET"
                    },
                    "ValueDetection": {
                        "Confidence": 99.97969,
                        "Geometry": {
                            "BoundingBox": {
                                "Height": 0.014895633,
                                "Left": 0.36269557,
                                "Top": 0.32430968,
                                "Width": 0.19572175
                            },
                            "Polygon": [
                                {
                                    "X": 0.36271322,
                                    "Y": 0.32535973
                                },
                                {
                                    "X": 0.5584173,
                                    "Y": 0.32430968
                                },
                                {
                                    "X": 0.55836666,
                                    "Y": 0.3381911
                                },
                                {
                                    "X": 0.36269557,
                                    "Y": 0.3392053
                                }
                            ]
                        },
                        "Text": "2717 ROCK ISLAND PL"
                    }
                },
                {
                    "GroupProperties": [
                        {
                            "Id": "9c053dc8-7cc1-4232-a8a4-04a188b0d047",
                            "Types": [
                                "VENDOR"
                            ]
                        }
                    ],
                    "PageNumber": 1,
                    "Type": {
                        "Confidence": 99.900925,
                        "Text": "CITY"
                    },
                    "ValueDetection": {
                        "Confidence": 99.99932,
                        "Geometry": {
                            "BoundingBox": {
                                "Height": 0.014395428,
                                "Left": 0.3731875,
                                "Top": 0.34045482,
                                "Width": 0.08384666
                            },
                            "Polygon": [
                                {
                                    "X": 0.37320712,
                                    "Y": 0.3408874
                                },
                                {
                                    "X": 0.4570342,
                                    "Y": 0.34045482
                                },
                                {
                                    "X": 0.45700037,
                                    "Y": 0.35443312
                                },
                                {
                                    "X": 0.3731875,
                                    "Y": 0.35485023
                                }
                            ]
                        },
                        "Text": "BISMARCK"
                    }
                },
                {
                    "GroupProperties": [
                        {
                            "Id": "9c053dc8-7cc1-4232-a8a4-04a188b0d047",
                            "Types": [
                                "VENDOR"
                            ]
                        }
                    ],
                    "PageNumber": 1,
                    "Type": {
                        "Confidence": 99.900925,
                        "Text": "STATE"
                    },
                    "ValueDetection": {
                        "Confidence": 99.99473,
                        "Geometry": {
                            "BoundingBox": {
                                "Height": 0.013546705,
                                "Left": 0.46717477,
                                "Top": 0.3402868,
                                "Width": 0.022418234
                            },
                            "Polygon": [
                                {
                                    "X": 0.46720892,
                                    "Y": 0.3404023
                                },
                                {
                                    "X": 0.489593,
                                    "Y": 0.3402868
                                },
                                {
                                    "X": 0.48955518,
                                    "Y": 0.35372195
                                },
                                {
                                    "X": 0.46717477,
                                    "Y": 0.3538335
                                }
                            ]
                        },
                        "Text": "ND"
                    }
                }
                ]
        }       
        "#; 

        let summary: SummaryFieldsWrapper  = serde_json::from_str(json_str).unwrap();

        let result = process_summary_fields(&summary.summary_fields);

        assert_eq!(result.len(), 4);

    }


}  



    

