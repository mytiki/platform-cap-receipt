/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod expense_field;
pub mod expense_field_response;
pub mod group;
pub mod group_response;
pub mod line_item;
pub mod line_item_response;

use group::Group;
use group_response::GroupResponse;

use crate::util::field_response::FieldResponse;

use self::{expense_field_response::ExpenseFieldResponse, line_item_response::LineItemResponse};

pub fn process(line_item_groups: &Vec<Group>) -> Vec<GroupResponse> {
    line_item_groups
        .iter()
        .map(|line_item_group| {
            GroupResponse {
                line_item_group_index: line_item_group.line_item_group_index,
                line_items: line_item_group
                    .line_items
                    .iter()
                    .map(|line_item| {
                        let line_item_expense_fields: Vec<ExpenseFieldResponse> = line_item
                            .line_item_expense_fields
                            .iter()
                            .map(|field| {
                                let mut expense_field = ExpenseFieldResponse {
                                    product_code: None,
                                    item: None,
                                    expense_row: None,
                                    price: None,
                                };
                                match field.line_item_expense_field_type.text.clone().unwrap().as_str() {
                                    "PRODUCT_CODE" => {
                                        expense_field.product_code =
                                            Some(FieldResponse {
                                                confidence_key: field
                                                    .line_item_expense_field_type
                                                    .confidence.unwrap_or(0.00),
                                                confidence_value: field.value_detection.confidence.unwrap_or(0.00),
                                                value: field.value_detection.text.clone().unwrap(),
                                            })
                                    }
                                    "ITEM" => {
                                        expense_field.item = Some(FieldResponse {
                                            confidence_key: field
                                                .line_item_expense_field_type
                                                .confidence.unwrap_or(0.00),
                                            confidence_value: field.value_detection.confidence.unwrap_or(0.00),
                                            value: field.value_detection.text.clone().unwrap(),
                                        })
                                    }
                                    "EXPENSE_ROW" => {
                                        expense_field.expense_row =
                                            Some(FieldResponse {
                                                confidence_key: field
                                                    .line_item_expense_field_type
                                                    .confidence.unwrap_or(0.00),
                                                confidence_value: field.value_detection.confidence.unwrap_or(0.00),
                                                value: field.value_detection.text.clone().unwrap(),
                                            })
                                    }
                                    "PRICE" => {
                                        expense_field.price = Some(FieldResponse {
                                            confidence_key: field
                                                .line_item_expense_field_type
                                                .confidence.unwrap_or(0.00),
                                            confidence_value: field.value_detection.confidence.unwrap_or(0.00),
                                            value: field.value_detection.text.clone().unwrap(),
                                        })
                                    }
                                    _ => {} // Handling other cases if needed
                                }
                                expense_field
                            })
                            .collect();
                        LineItemResponse {
                            line_item_expense_fields,
                        }
                    })
                    .collect(),
            }
        })
        .collect()
}

#[test]
fn test_process_line_items() {
    let json_str = r#" {
            "GroupIndex": 1,
            "LineItems": [
                {
                    "ExpenseFields": [
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
                    "ExpenseFields": [
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
                    "ExpenseFields": [
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

    let line_items: Group = serde_json::from_str(json_str).unwrap();

    let result = process(&vec![line_items]);

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
                assert_eq!(summary_field.value, "6 WING PLATE 020108870398 3.98 P");
            }
            None => {}
        }
    }
}
