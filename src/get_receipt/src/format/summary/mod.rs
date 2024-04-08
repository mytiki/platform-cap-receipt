/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

pub mod field;
pub mod field_wrapper;
pub mod group_property;

use std::collections::HashMap;

use field::SummaryField;

use crate::util::field_response::FieldResponse;

pub fn process(summary_fields: &[SummaryField]) -> Vec<HashMap<String, FieldResponse>> {
  summary_fields
      .iter()
      .map(|summary_field| {
          let mut summary_map = HashMap::new();

          let key = match summary_field
            .summary_field_type{
              Some(ref summary_field_type) => summary_field_type.text.as_ref().unwrap().to_owned(),
              None => {
                if summary_field.label_detection.is_some() {
                  "OTHER"
                } else {
                  ""
                }
              }.to_owned()
          };

          let confidence_key = match summary_field.summary_field_type {
              Some(ref summary_field_type) => summary_field_type.confidence.unwrap_or(0.00) as f64, // Changed to f64
              None => summary_field.label_detection.as_ref().map_or(0.0, |ld| ld.confidence.unwrap_or(0.00) as f64), // Changed to f64
          };

          summary_map.insert(
              key,
              FieldResponse {
                  confidence_key,
                  confidence_value: summary_field.value_detection.confidence.unwrap_or(0.00),
                  value: summary_field.value_detection.text.clone().unwrap(),
              },
          );

          summary_map
      })
      .collect()

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

    let summary: field_wrapper::SummaryFieldsWrapper = serde_json::from_str(json_str).unwrap();

    let result = process(&summary.summary_fields);

    assert_eq!(result.len(), 4);

}

