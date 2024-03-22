// use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
// use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct TextractInput {
//     document_metadata: DocumentMetadata,
//     expense_documents: Vec<ExpenseDocument>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct DocumentMetadata {
//     pages: i64,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct ExpenseDocument {
//     blocks: Vec<Block>,
//     expense_index: i64,
//     line_item_groups: Vec<LineItemGroup>,
//     summary_fields: Vec<SummaryField>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Block {
//     block_type: BlockType,
//     geometry: Geometry,
//     id: String,
//     relationships: Option<Vec<Relationship>>,
//     confidence: Option<f64>,
//     text: Option<String>,
//     page: Option<i64>,
//     text_type: Option<TextType>,
// }

// #[derive(Serialize, Deserialize)]
// pub enum BlockType {
//     #[serde(rename = "LINE")]
//     Line,
//     #[serde(rename = "PAGE")]
//     Page,
//     #[serde(rename = "WORD")]
//     Word,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Geometry {
//     bounding_box: BoundingBox,
//     polygon: Vec<Polygon>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct BoundingBox {
//     height: f64,
//     left: f64,
//     top: f64,
//     width: f64,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub struct Polygon {
//     x: f64,
//     y: f64,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Relationship {
//     ids: Vec<String>,
//     #[serde(rename = "Type")]
//     relationship_type: TypeEnum,
// }

// #[derive(Serialize, Deserialize)]
// pub enum TypeEnum {
//     #[serde(rename = "CHILD")]
//     Child,
// }

// #[derive(Serialize, Deserialize)]
// pub enum TextType {
//     #[serde(rename = "PRINTED")]
//     Printed,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct SummaryField {
//     group_properties: Option<Vec<GroupProperty>>,
//     page_number: i64,
//     #[serde(rename = "Type")]
//     summary_field_type: Option<TypeClass>,
//     value_detection: Detection,
//     label_detection: Option<Detection>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct GroupProperty {
//     id: String,
//     types: Vec<String>,
// }


// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct StateResponse {
//     document_metadata: DocumentMetadata,
//     expense_documents: Vec<StateResponseExpenseDocument>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct StateResponseExpenseDocument {
//     blocks: Vec<StateResponseBlock>,
//     expense_index: i64,
//     line_item_groups: Vec<LineItemGroup>,
//     summary_fields: Vec<HashMap<String, StateResponseSummaryField>>,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct StateResponseBlock {
//     confidence: f64,
//     text: String,
// }




// use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
// use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

// // Function to process line item groups
// fn process_line_item_groups(line_item_groups: &Vec<LineItemGroup>) -> Vec<StateResponseLineItemGroup> {

//     let mut line_items_map = HashMap::new();

//     line_item_groups.iter().map(|line_item_group| {

//         let line_items = line_item_group.line_items.iter().map(|line_item| {

//             let line_item_expense_fields = line_item.line_item_expense_fields.iter().map(|field| {
//                 field.clone().iter().map(|sf| {
//                     line_items_map.insert(sf.Type.Text.clone(), StateResponseSummaryField {
//                         confidence_key: sf.confidence.clone(),
//                         confidence_value: sf.value_detection.confidence.clone(), // Placeholder value, adjust accordingly
//                         value: sf.value_detection.text.clone(),
//                     });
//                 });

//                 StateResponseLineItemExpenseField {
//                     line_items_map
//                 }
//             }).collect();

//             StateResponseLineItem {
//                 line_item_expense_fields,
//             }
//         }).collect();

//         StateResponseLineItemGroup {
//             line_item_group_index: line_item_group.line_item_group_index,
//             line_items,
//         }
//     }).collect()
// }

// // Function to process summary fields
// fn process_summary_fields(summary_fields: &[SummaryField]) -> Vec<HashMap<String, StateResponseSummaryField>> {
//     summary_fields.iter().map(|summary_field| {

//         let mut summary_map = HashMap::new();

//         let key = match summary_field.summary_field_type.text.as_str() {
//             "OTHER" => summary_field.label_detection.as_ref().map_or("", |ld| ld.text.as_str()).to_string(),
//             _ => summary_field.summary_field_type.text.clone(),
//         };
        
//         let confidence_key = match (field.summary_field_type){
//             Some(field_type) => field_type.confidence,
//             None => field.label_detection.confidence
//         };

//         summary_map.insert(key, StateResponseSummaryField {
//             confidence_key: confidence_key,
//             confidence_value: summary_field.value_detection.confidence.clone(),
//             value: summary_field.value_detection.text.clone(),
//         });

//         summary_map
//     }).collect()
// }

// // Main function
// async fn function_handler(event: &LambdaEvent<TextractInput>) -> Result<StateResponse, Error> {
//     // Extracting data from TextractInput
//     let document_metadata = event.payload.document_metadata;
//     let expense_documents = event.payload.expense_documents.iter().map(|expense_doc| {
//         // Processing blocks
//         let blocks = process_blocks(&expense_doc.blocks);
//         // Processing line item groups
//         let line_item_groups = process_line_item_groups(&expense_doc.line_item_groups);
//         // Processing summary fields
//         let summary_fields = process_summary_fields(&expense_doc.summary_fields);

//         // Constructing ExpenseDocument
//         ExpenseDocument {
//             blocks,
//             expense_index: expense_doc.expense_index,
//             line_item_groups,
//             summary_fields,
//         }
//     }).collect();

//     // Constructing StateResponse
//     let state_response = StateResponse {
//         document_metadata,
//         expense_documents,
//     };

//     Ok(state_response)
// }


// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     tracing::init_default_subscriber();

//     run(service_fn(function_handler)).await
// }


use serde::{self, Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    block_type: BlockType,
    geometry: Geometry,
    id: String,
    relationships: Vec<Relationship>,
    confidence: f64,
    text: String,
    page: Option<i64>,
    text_type: Option<TextType>,
}

#[derive(Serialize, Deserialize)]
pub enum BlockType {
    #[serde(rename = "LINE")]
    Line,
    #[serde(rename = "PAGE")]
    Page,
    #[serde(rename = "WORD")]
    Word,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Geometry {
    bounding_box: BoundingBox,
    polygon: Vec<Polygon>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoundingBox {
    height: f64,
    left: f64,
    top: f64,
    width: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Polygon {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Relationship {
    ids: Vec<String>,
    #[serde(rename = "Type")]
    relationship_type: TypeEnum,
}

#[derive(Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "CHILD")]
    Child,
}

#[derive(Serialize, Deserialize)]
pub enum TextType {
    #[serde(rename = "PRINTED")]
    Printed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseBlock {
    confidence: f64,
    text: String,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItemGroup {
    line_item_group_index: i64,
    line_items: Vec<LineItem>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItem {
    line_item_expense_fields: Vec<LineItemExpenseField>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItemExpenseField {
    page_number: i64,
    #[serde(rename = "line_item_type")]
    line_item_expense_field_type: TypeClass,
    value_detection: Detection,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TypeClass {
    confidence: f64,
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Detection {
    confidence: f64,
    geometry: Geometry,
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseLineItemGroup {
    line_item_group_index: i64,
    line_items: Vec<StateResponseLineItem>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseLineItem {
    line_item_expense_fields: Vec<StateResponseLineItemExpenseField>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateResponseLineItemExpenseField {
    product_code: Option<StateResponseSummaryField>,
    item: Option<StateResponseSummaryField>,
    price: Option<StateResponseSummaryField>,
    expense_row: Option<StateResponseSummaryField>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateResponseSummaryField {
    confidence_key: f64,
    confidence_value: f64,
    value: String,
}


// Function to process blocks
fn process_blocks(blocks: &Vec<Block>) -> Vec<StateResponseBlock> {
    blocks.iter().map(|block| {
        StateResponseBlock{
            confidence: block.confidence,
            text: block.text.clone(),
        }
    }).collect()
} 

// Function to process LineItems
fn process_line_item_groups(line_item_groups: &Vec<LineItemGroup>) -> Vec<StateResponseLineItemGroup> {

    let mut line_items_vec = Vec::new();
    line_item_groups.iter().map(|line_item_group| {

        let line_items = line_item_group.line_items.iter().map(|line_item| {

            let mut expense_field = StateResponseLineItemExpenseField;

            let line_item_expense_fields = line_item.line_item_expense_fields.iter().map(|field| {

                field.clone().iter().map(|sf| {
                    
                    match sf.line_item_type.text {
                        "PRODUCT_CODE" => expense_field.product_code = Some(StateResponseSummaryField{
                            
                                confidence_key: sf.line_item_type.confidence,
                                confidence_value: sf.value_detection.confidence,
                                value: sf.value_detection.text,
                            
                        }),
                        "ITEM" => expense_field.item = Some(StateResponseSummaryField{
                                confidence_key: sf.line_item_type.confidence,
                                confidence_value: sf.value_detection.confidence,
                                value: sf.value_detection.text,
                        }),
                        "EXPENSE_ROW" => expense_field.expense_row = Some(StateResponseSummaryField{
                                confidence_key: sf.line_item_type.confidence,
                                confidence_value: sf.value_detection.confidence,
                                value: sf.value_detection.text,
                        }),
                        "PRICE" => expense_field.price = Some(StateResponseSummaryField{
                                confidence_key: sf.line_item_type.confidence,
                                confidence_value: sf.value_detection.confidence,
                                value: sf.value_detection.text,
                        })
                    }
                    // line_items_vec.push(expense_field)
                });

                StateResponseLineItemExpenseField {
                    ..expense_field
                }
            }).collect();

            StateResponseLineItem {
                line_items_vec,
            }
        }).collect();

        StateResponseLineItemGroup {
            line_item_group_index: line_item_group.line_item_group_index,
            line_items,
        }
    }).collect()
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
        let json_str = r#"
        "LineItemGroups": [
            {
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
        ],
        "#;
        
        let lineItems: LineItemGroup = serde_json::from_str(json_str).unwrap();

        let result = process_line_item_groups(&vec![lineItems]);

      
        }
}  

    

