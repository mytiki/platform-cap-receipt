/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use std::error::Error;

use lambda_http::aws_lambda_events::apigw::serialize_authorizer_fields;

use crate::{util::{input_data::{self, InputData}, output_data::OutputData}};

pub mod block;
pub mod geometry;
pub mod line_item;
pub mod summary;
pub mod type_class;

pub fn process(data: InputData) -> Result<String, Box<dyn Error>> {
    let processed_blocks = block::process(&data.expense_documents[0].blocks);

    let processed_line_item_groups = line_item::process(&data.expense_documents[0].line_item_groups);

    let processed_summary_fields = summary::process(&data.expense_documents[0].summary_fields);

    let result = OutputData{
      blocks:  processed_blocks,
      line_item_groups: processed_line_item_groups, 
      summary_fields:  processed_summary_fields
    };
    let json_string = serde_json::to_string(&result)?;
    
    Ok(json_string)
}

#[test]
fn test_process() {
  let json = r#"
  {
    "DocumentMetadata": {
      "Pages": 1
    },
    "ExpenseDocuments": [
      {
        "Blocks": [
          {
            "BlockType": "PAGE",
            "Geometry": {
              "BoundingBox": {
                "Height": 1.0,
                "Left": 0.06945638,
                "Top": 0.0,
                "Width": 0.78810036
              },
              "Polygon": [
                {
                  "X": 0.10241407,
                  "Y": 0.0
                },
                {
                  "X": 0.85755676,
                  "Y": 0.0
                },
                {
                  "X": 0.8240894,
                  "Y": 1.0
                },
                {
                  "X": 0.06945638,
                  "Y": 1.0
                }
              ]
            },
            "Id": "2b180b77-7f5b-4200-9082-8450cea0fc46",
            "Relationships": [
              {
                "Ids": [
                  "0c44b956-ebf6-46b3-97b8-76d91878792d",
                  "3aa833d0-06d8-474c-ac8e-ddc56e77bbfc",
                  "993ebf32-b325-4f0c-932f-67e8833ad390",
                  "ad62780e-b7a8-4856-8c5d-185024303068",
                  "60a257cb-be58-4a21-b271-02df043fed81",
                  "88c3ed61-da4a-4047-adc7-275a4e245e7f",
                  "24cb3be4-1c46-44cc-b929-6da1cbb005ff",
                  "2808a0a6-e913-4777-83fb-3df83edb53b2",
                  "4b49fdbb-1e70-456c-bd3d-6dbd6512b4ab",
                  "37b52b7c-5ed7-4f31-9224-9ea772f55f42",
                  "ece04039-afca-4f3d-bd19-4311244718ab",
                  "c89e0713-fdc1-4c2e-92a5-e43284a7d844",
                  "2ccbc0b9-478a-4d64-879c-3f2d62bf9c36",
                  "31fad142-3647-4b58-8dbf-ae9ed32fe1fc",
                  "616f6744-4e48-4706-ae5f-d20a80d12212",
                  "d2541d02-f324-42a0-9556-826b2860c7a8",
                  "39442b4e-ba6f-4c7e-8546-f6aaedfc266b",
                  "ac1c8ae2-445b-4709-af64-755d6a980091",
                  "f9ae50d1-00a8-4c8b-a040-d1bdb8e8b469",
                  "f28ae6d0-f373-462f-8de0-cf9a8ccd701d",
                  "f7962eb6-edb0-4ce9-94b4-ebb644ee6ce2",
                  "eed6a1de-70bc-41af-93db-f2c71566ce73",
                  "3b7bf82e-cdfc-40b0-8cec-a125cf841462",
                  "322d3179-d7fb-4d59-bd85-b34e1d474b14",
                  "3e75a2c0-0fcc-47fd-b13c-a8110fd8c0ec",
                  "f310a91a-fecf-4cec-8652-45ff6325503e",
                  "5b4897a4-e4fb-4599-a9de-2f218a4972a6",
                  "5597e6e2-1c4f-427c-8c05-64754e10a55e",
                  "ffab194e-5506-46d3-b6e4-a86f66e1e951",
                  "786aa80d-4e9f-4fb7-ad0c-710dc2265b04",
                  "6f0ef31f-c8cb-48cd-a65b-b33c3cdfa0d0",
                  "0cf1fce8-e93b-467d-bbd6-288345048259",
                  "7f067cc2-1c8b-42fd-a65d-d5608aa2c284",
                  "ea1cc434-5beb-40c5-a678-e53090d41c65",
                  "86015541-908b-4e87-b817-fa528e980b9c",
                  "7a28cc9d-cd79-4625-b461-4521367ae421",
                  "74c5385d-b9b5-4aff-9a48-d433cba012e4",
                  "65c9697e-5033-4528-8c5b-83cdfc7f4a5c",
                  "a55c834d-66fd-4389-a6f4-99832796bd0e",
                  "b138bf94-3c75-49f3-9e99-db33fadf6fe4",
                  "e41e6dcf-0bb4-4ef6-a7c6-fed797ffac9a",
                  "4964345b-5ad3-435b-908d-aab385499bf6",
                  "fe9fa440-320e-4272-ae68-0c843cbd652f",
                  "48473399-452c-4bc1-912f-70821b29b9f4",
                  "70b064da-afce-41e4-8292-a648bbfc80ea",
                  "57f3bcf8-e7e6-4cca-b9fe-b9f218236b00",
                  "ba21923f-58e5-4836-a0ce-8d7248380997",
                  "087bcf0c-938f-4c05-a177-c1dc66fe096e",
                  "c26361cb-450f-4d9a-b220-71bb5cc530e0",
                  "d47b4a25-e9c1-450b-a02c-ab51a57da37b",
                  "e64f087e-cb69-42f4-b01b-6950c022597c",
                  "31857a6b-4130-47ff-8b00-4752a18efc4b",
                  "55bb4050-e9a9-4177-93ce-e44c8f6c9615",
                  "5cdd5351-7b51-43d6-9a6b-7ab178f5ed7f",
                  "08ee383e-05c2-43b1-96c9-690e6e955e8e",
                  "6e289678-886d-450f-86e7-f19b28e5799c",
                  "68006f3d-04fb-47b2-bccd-f7bf1120efec",
                  "dc5a16dc-cd3c-4f5e-bbf2-ee355c72e3ba",
                  "74c62dd8-47ad-4c49-9f73-9aa99129ea87",
                  "3e0678ac-429f-4c4d-8009-18c0335f3f09",
                  "4f1f554a-029e-47ef-92d4-58ceb57c4e2a",
                  "7feb9660-c2e4-4cf3-8b26-6622584ae7ce",
                  "82469eae-4852-4058-8406-deee87e9f0fc",
                  "63f6e1bb-133a-4afe-82c3-dff46be251b6",
                  "cb03544a-f3b8-427e-88f6-d829c5b13105",
                  "73626b59-3e04-4255-bb27-3b1e377b7568",
                  "cff60520-ce36-4478-a59a-0777d9241bdd",
                  "28dc7580-d92f-485a-b488-fc5ddd2370ac",
                  "82ed0a28-73c2-4355-b77c-ac9504f5c921"
                ],
                "Type": "CHILD"
              }
            ]
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.35089,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.066532075,
                "Left": 0.27903962,
                "Top": 0.025094835,
                "Width": 0.37162614
              },
              "Polygon": [
                {
                  "X": 0.2811996,
                  "Y": 0.025094835
                },
                {
                  "X": 0.65066576,
                  "Y": 0.02584732
                },
                {
                  "X": 0.6484895,
                  "Y": 0.09162691
                },
                {
                  "X": 0.27903962,
                  "Y": 0.09073055
                }
              ]
            },
            "Id": "0c44b956-ebf6-46b3-97b8-76d91878792d",
            "Relationships": [
              {
                "Ids": [
                  "67e23304-4ea9-4077-8cb3-2c477bf5858b"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Michaels"
          },
          {
            "BlockType": "LINE",
            "Confidence": 90.84586,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.025955897,
                "Left": 0.34560236,
                "Top": 0.10280453,
                "Width": 0.24194303
              },
              "Polygon": [
                {
                  "X": 0.34643635,
                  "Y": 0.10280453
                },
                {
                  "X": 0.5875454,
                  "Y": 0.10340655
                },
                {
                  "X": 0.5867073,
                  "Y": 0.12876043
                },
                {
                  "X": 0.34560236,
                  "Y": 0.1281222
                }
              ]
            },
            "Id": "3aa833d0-06d8-474c-ac8e-ddc56e77bbfc",
            "Relationships": [
              {
                "Ids": [
                  "63306dca-7a1d-4363-993b-9f867b49088f",
                  "a5cbac67-046a-43e2-a212-fc5e916310b4",
                  "6203258b-a91e-4e59-9eef-d76abbe122e4"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Made by you"
          },
          {
            "BlockType": "LINE",
            "Confidence": 92.82967,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.026953135,
                "Left": 0.23805149,
                "Top": 0.12455209,
                "Width": 0.45519498
              },
              "Polygon": [
                {
                  "X": 0.23889643,
                  "Y": 0.12455209
                },
                {
                  "X": 0.6932465,
                  "Y": 0.12574592
                },
                {
                  "X": 0.69239366,
                  "Y": 0.15150523
                },
                {
                  "X": 0.23805149,
                  "Y": 0.15024212
                }
              ]
            },
            "Id": "993ebf32-b325-4f0c-932f-67e8833ad390",
            "Relationships": [
              {
                "Ids": [
                  "d27420a0-78d2-4258-8568-7bcee7a3872a",
                  "f899b08c-713c-4d49-8ed1-1c77b4388ec3",
                  "e1a99d1d-e673-409c-b5d5-05c1bc81d7b5",
                  "a560b0b3-5f42-4461-95ba-0564b734925b",
                  "7e8db4fd-6d52-440d-9580-b6b3516135f3"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "MICHAELS STORE #9010 (386) 767-7495"
          },
          {
            "BlockType": "LINE",
            "Confidence": 90.81743,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018682597,
                "Left": 0.33007756,
                "Top": 0.14724608,
                "Width": 0.25524938
              },
              "Polygon": [
                {
                  "X": 0.3306688,
                  "Y": 0.14724608
                },
                {
                  "X": 0.5853269,
                  "Y": 0.14794913
                },
                {
                  "X": 0.5847326,
                  "Y": 0.16592868
                },
                {
                  "X": 0.33007756,
                  "Y": 0.1651985
                }
              ]
            },
            "Id": "ad62780e-b7a8-4856-8c5d-185024303068",
            "Relationships": [
              {
                "Ids": [
                  "8c177e41-a642-418f-a010-4a0970b5195b",
                  "e92faa69-8be2-4ec0-8c9f-39e7431e11ed",
                  "3fffd05c-2d78-4531-877b-9f65bf0d80d1"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "MICHAELS STORE #9010"
          },
          {
            "BlockType": "LINE",
            "Confidence": 81.21385,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.020866103,
                "Left": 0.320172,
                "Top": 0.1590581,
                "Width": 0.27411777
              },
              "Polygon": [
                {
                  "X": 0.32083255,
                  "Y": 0.1590581
                },
                {
                  "X": 0.5942898,
                  "Y": 0.15983225
                },
                {
                  "X": 0.59362555,
                  "Y": 0.1799242
                },
                {
                  "X": 0.320172,
                  "Y": 0.1791175
                }
              ]
            },
            "Id": "60a257cb-be58-4a21-b271-02df043fed81",
            "Relationships": [
              {
                "Ids": [
                  "40482d00-1bc8-42ec-a357-33beec61ba9c",
                  "123d99a7-ef99-4812-b968-7af05ded094c",
                  "2575c6fa-1269-4863-b771-9961c0427a63",
                  "4cb5d9ab-9417-4131-a9ff-be9aceae83ee",
                  "8b4073f0-a834-45cd-9eca-ad0916a41de9"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "5507 S WILL IAMSON BLVD"
          },
          {
            "BlockType": "LINE",
            "Confidence": 63.613014,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0223665,
                "Left": 0.31827524,
                "Top": 0.17269175,
                "Width": 0.27350566
              },
              "Polygon": [
                {
                  "X": 0.31898445,
                  "Y": 0.17269175
                },
                {
                  "X": 0.5917809,
                  "Y": 0.17348611
                },
                {
                  "X": 0.5910678,
                  "Y": 0.19505826
                },
                {
                  "X": 0.31827524,
                  "Y": 0.19422904
                }
              ]
            },
            "Id": "88c3ed61-da4a-4047-adc7-275a4e245e7f",
            "Relationships": [
              {
                "Ids": [
                  "a00b601f-c856-4598-aefa-a148240d0c42",
                  "3c597335-c575-4cb8-8b88-42a2e4676bf1",
                  "ad92a06e-478e-4206-95cc-57ad25ca9bd8",
                  "2dd205fd-b2b2-455e-a8bd-749376a227b1"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "PORT ORGANCE FL 0328"
          },
          {
            "BlockType": "LINE",
            "Confidence": 75.757126,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.02366206,
                "Left": 0.267229,
                "Top": 0.18523951,
                "Width": 0.2729528
              },
              "Polygon": [
                {
                  "X": 0.26797962,
                  "Y": 0.18523951
                },
                {
                  "X": 0.54018176,
                  "Y": 0.18605265
                },
                {
                  "X": 0.539427,
                  "Y": 0.20890157
                },
                {
                  "X": 0.267229,
                  "Y": 0.20805158
                }
              ]
            },
            "Id": "24cb3be4-1c46-44cc-b929-6da1cbb005ff",
            "Relationships": [
              {
                "Ids": [
                  "883b7cd8-6644-4d07-ae16-7be3d58dbbf0",
                  "dad38341-6710-43c1-bb3d-b351152035eb",
                  "c977b83f-7199-435c-81e9-e9d0e9f2588b"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Rewards Number CARROT"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.65637,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017512016,
                "Left": 0.16682796,
                "Top": 0.27234024,
                "Width": 0.17762236
              },
              "Polygon": [
                {
                  "X": 0.16738239,
                  "Y": 0.27234024
                },
                {
                  "X": 0.34445032,
                  "Y": 0.27296108
                },
                {
                  "X": 0.3438939,
                  "Y": 0.28985226
                },
                {
                  "X": 0.16682796,
                  "Y": 0.28921366
                }
              ]
            },
            "Id": "2808a0a6-e913-4777-83fb-3df83edb53b2",
            "Relationships": [
              {
                "Ids": [
                  "46034d61-d5b3-4987-8af2-3f8861f17396",
                  "cd3049f1-8db3-491a-9e27-6092bdd8d2ac"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "4033602 SALE"
          },
          {
            "BlockType": "LINE",
            "Confidence": 98.75079,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.029104937,
                "Left": 0.41661885,
                "Top": 0.2663284,
                "Width": 0.3701177
              },
              "Polygon": [
                {
                  "X": 0.41753447,
                  "Y": 0.2663284
                },
                {
                  "X": 0.78673655,
                  "Y": 0.26760784
                },
                {
                  "X": 0.78581405,
                  "Y": 0.2954333
                },
                {
                  "X": 0.41661885,
                  "Y": 0.2940931
                }
              ]
            },
            "Id": "4b49fdbb-1e70-456c-bd3d-6dbd6512b4ab",
            "Relationships": [
              {
                "Ids": [
                  "dc20095e-e17a-4ff8-86b3-1b4a2b98ebf7",
                  "8c410fc1-6c99-4b14-a0f9-436fdebc5f3b",
                  "159fcc1c-f3cc-46b0-9be8-f1c1d81ae13c",
                  "f33a2726-f2ea-47c4-a0b0-8756ebc686ef",
                  "240e0ec6-8507-46fe-bca0-0b2595b94365"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "0659 9010 002 1/04/22 13:03"
          },
          {
            "BlockType": "LINE",
            "Confidence": 74.425804,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018962525,
                "Left": 0.12356567,
                "Top": 0.28576893,
                "Width": 0.22331622
              },
              "Polygon": [
                {
                  "X": 0.12416134,
                  "Y": 0.28576893
                },
                {
                  "X": 0.3468819,
                  "Y": 0.2865678
                },
                {
                  "X": 0.3462835,
                  "Y": 0.30473146
                },
                {
                  "X": 0.12356567,
                  "Y": 0.30390856
                }
              ]
            },
            "Id": "37b52b7c-5ed7-4f31-9224-9ea772f55f42",
            "Relationships": [
              {
                "Ids": [
                  "e45bde78-4999-4bc4-810c-c4f4e84e83c3",
                  "4db95470-704d-4e69-9902-101597525a13",
                  "d46616be-3162-4cb7-9442-cdef40bb444f"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "CNCL 2002 HOLIDAY"
          },
          {
            "BlockType": "LINE",
            "Confidence": 95.526764,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015371156,
                "Left": 0.37924334,
                "Top": 0.29258725,
                "Width": 0.15386045
              },
              "Polygon": [
                {
                  "X": 0.3797312,
                  "Y": 0.29258725
                },
                {
                  "X": 0.53310376,
                  "Y": 0.29314277
                },
                {
                  "X": 0.53261435,
                  "Y": 0.3079584
                },
                {
                  "X": 0.37924334,
                  "Y": 0.30738944
                }
              ]
            },
            "Id": "ece04039-afca-4f3d-bd19-4311244718ab",
            "Relationships": [
              {
                "Ids": [
                  "2568672f-1cc0-4652-93b0-1a37464206a1"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "647658036793"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.88898,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015371211,
                "Left": 0.57754713,
                "Top": 0.2893452,
                "Width": 0.06596329
              },
              "Polygon": [
                {
                  "X": 0.5780472,
                  "Y": 0.2893452
                },
                {
                  "X": 0.6435104,
                  "Y": 0.28958076
                },
                {
                  "X": 0.64300966,
                  "Y": 0.3047164
                },
                {
                  "X": 0.57754713,
                  "Y": 0.30447498
                }
              ]
            },
            "Id": "c89e0713-fdc1-4c2e-92a5-e43284a7d844",
            "Relationships": [
              {
                "Ids": [
                  "edd4135e-c7a2-4018-9804-c7cebd603c38"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "19.99"
          },
          {
            "BlockType": "LINE",
            "Confidence": 51.185585,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015274781,
                "Left": 0.47886223,
                "Top": 0.3063904,
                "Width": 0.076895446
              },
              "Polygon": [
                {
                  "X": 0.47935686,
                  "Y": 0.3063904
                },
                {
                  "X": 0.5557577,
                  "Y": 0.3066732
                },
                {
                  "X": 0.55526227,
                  "Y": 0.32166517
                },
                {
                  "X": 0.47886223,
                  "Y": 0.32137558
                }
              ]
            },
            "Id": "2ccbc0b9-478a-4d64-879c-3f2d62bf9c36",
            "Relationships": [
              {
                "Ids": [
                  "682cc7e1-2cef-4c8e-8e31-7bd0b0cba14c"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "6.00"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.74971,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.019835003,
                "Left": 0.6894387,
                "Top": 0.2968977,
                "Width": 0.09644067
              },
              "Polygon": [
                {
                  "X": 0.6900835,
                  "Y": 0.2968977
                },
                {
                  "X": 0.7858794,
                  "Y": 0.2972465
                },
                {
                  "X": 0.7852334,
                  "Y": 0.3167327
                },
                {
                  "X": 0.6894387,
                  "Y": 0.3163729
                }
              ]
            },
            "Id": "31fad142-3647-4b58-8dbf-ae9ed32fe1fc",
            "Relationships": [
              {
                "Ids": [
                  "e1b07e60-6735-4f77-b305-ba7d1c2bcb9c",
                  "18fc26d6-4d10-4c67-a5f9-876861c9ed7f"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "6.00 P"
          },
          {
            "BlockType": "LINE",
            "Confidence": 96.27628,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.019348936,
                "Left": 0.12383525,
                "Top": 0.31504583,
                "Width": 0.2227326
              },
              "Polygon": [
                {
                  "X": 0.124442406,
                  "Y": 0.31504583
                },
                {
                  "X": 0.34656784,
                  "Y": 0.31588122
                },
                {
                  "X": 0.34595793,
                  "Y": 0.33439475
                },
                {
                  "X": 0.12383525,
                  "Y": 0.333535
                }
              ]
            },
            "Id": "616f6744-4e48-4706-ae5f-d20a80d12212",
            "Relationships": [
              {
                "Ids": [
                  "d3a8346a-6068-4bfc-9d09-5549b31b5de3",
                  "cbdbd556-d753-41f0-9cf8-0fb10ce276e7",
                  "54b54b28-b572-4618-820e-6bf2b6032387",
                  "c8d21f69-9598-49f5-9bf7-57e4d60ec1a0"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "ST TREND STYLE PH"
          },
          {
            "BlockType": "LINE",
            "Confidence": 98.69644,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014908325,
                "Left": 0.38079435,
                "Top": 0.3207849,
                "Width": 0.15234108
              },
              "Polygon": [
                {
                  "X": 0.38126636,
                  "Y": 0.3207849
                },
                {
                  "X": 0.5331354,
                  "Y": 0.32136035
                },
                {
                  "X": 0.532662,
                  "Y": 0.3356932
                },
                {
                  "X": 0.38079435,
                  "Y": 0.33510485
                }
              ]
            },
            "Id": "d2541d02-f324-42a0-9556-826b2860c7a8",
            "Relationships": [
              {
                "Ids": [
                  "2cad7d51-a29f-4489-b466-18493dce4881"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "192040076524"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.83552,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016229672,
                "Left": 0.5878109,
                "Top": 0.31791732,
                "Width": 0.05546808
              },
              "Polygon": [
                {
                  "X": 0.5883404,
                  "Y": 0.31791732
                },
                {
                  "X": 0.64327896,
                  "Y": 0.3181243
                },
                {
                  "X": 0.64274883,
                  "Y": 0.33414698
                },
                {
                  "X": 0.5878109,
                  "Y": 0.33393478
                }
              ]
            },
            "Id": "39442b4e-ba6f-4c7e-8546-f6aaedfc266b",
            "Relationships": [
              {
                "Ids": [
                  "3446d04b-eece-4a63-b3b4-086ce5b7eb73"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "5.99"
          },
          {
            "BlockType": "LINE",
            "Confidence": 72.24644,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01617761,
                "Left": 0.45100057,
                "Top": 0.33399904,
                "Width": 0.104061194
              },
              "Polygon": [
                {
                  "X": 0.45152083,
                  "Y": 0.33399904
                },
                {
                  "X": 0.55506176,
                  "Y": 0.3343993
                },
                {
                  "X": 0.55454046,
                  "Y": 0.35017663
                },
                {
                  "X": 0.45100057,
                  "Y": 0.34976667
                }
              ]
            },
            "Id": "ac1c8ae2-445b-4709-af64-755d6a980091",
            "Relationships": [
              {
                "Ids": [
                  "0af86acf-9911-4764-a8b2-2c62b72e9a83"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "2@2.99"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.00594,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017252626,
                "Left": 0.6886747,
                "Top": 0.3283528,
                "Width": 0.09696607
              },
              "Polygon": [
                {
                  "X": 0.6892333,
                  "Y": 0.3283528
                },
                {
                  "X": 0.7856407,
                  "Y": 0.32872176
                },
                {
                  "X": 0.78508097,
                  "Y": 0.34560543
                },
                {
                  "X": 0.6886747,
                  "Y": 0.34522685
                }
              ]
            },
            "Id": "f9ae50d1-00a8-4c8b-a040-d1bdb8e8b469",
            "Relationships": [
              {
                "Ids": [
                  "1e785c54-01db-40bc-b9b6-99f9bcbdc97e",
                  "be9c9b05-6747-4a7a-9a4f-b7130ad0f13b"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "5.98 P"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.37627,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018768437,
                "Left": 0.12337482,
                "Top": 0.3464798,
                "Width": 0.21891984
              },
              "Polygon": [
                {
                  "X": 0.12396209,
                  "Y": 0.3464798
                },
                {
                  "X": 0.34229466,
                  "Y": 0.34734172
                },
                {
                  "X": 0.34170476,
                  "Y": 0.36524823
                },
                {
                  "X": 0.12337482,
                  "Y": 0.36436316
                }
              ]
            },
            "Id": "f28ae6d0-f373-462f-8de0-cf9a8ccd701d",
            "Relationships": [
              {
                "Ids": [
                  "f3e93eed-a921-45e1-a017-2eda28e0f611",
                  "78ba43bd-5403-4073-90cb-7ba83e0b252c",
                  "7f52c6e8-f832-4a61-80e7-b3d595e128ec"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "GA LINSEED REFINE"
          },
          {
            "BlockType": "LINE",
            "Confidence": 81.64376,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015264325,
                "Left": 0.38062787,
                "Top": 0.3504369,
                "Width": 0.14757542
              },
              "Polygon": [
                {
                  "X": 0.38111138,
                  "Y": 0.3504369
                },
                {
                  "X": 0.5282033,
                  "Y": 0.35102013
                },
                {
                  "X": 0.52771837,
                  "Y": 0.36570123
                },
                {
                  "X": 0.38062787,
                  "Y": 0.3651052
                }
              ]
            },
            "Id": "f7962eb6-edb0-4ce9-94b4-ebb644ee6ce2",
            "Relationships": [
              {
                "Ids": [
                  "ebc0dde1-1dc4-40d1-bd18-c61d2cabfffa"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "T29911060087"
          },
          {
            "BlockType": "LINE",
            "Confidence": 78.80073,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015898459,
                "Left": 0.5777256,
                "Top": 0.34852678,
                "Width": 0.0659237
              },
              "Polygon": [
                {
                  "X": 0.5782424,
                  "Y": 0.34852678
                },
                {
                  "X": 0.6436493,
                  "Y": 0.34878507
                },
                {
                  "X": 0.64313185,
                  "Y": 0.36442524
                },
                {
                  "X": 0.5777256,
                  "Y": 0.3641609
                }
              ]
            },
            "Id": "eed6a1de-70bc-41af-93db-f2c71566ce73",
            "Relationships": [
              {
                "Ids": [
                  "44f2f3cc-552d-47e2-af0f-5a1ae96f0d7b"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "15.99"
          },
          {
            "BlockType": "LINE",
            "Confidence": 67.3431,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015091474,
                "Left": 0.45205012,
                "Top": 0.36380962,
                "Width": 0.11495086
              },
              "Polygon": [
                {
                  "X": 0.45253247,
                  "Y": 0.36380962
                },
                {
                  "X": 0.567001,
                  "Y": 0.36427236
                },
                {
                  "X": 0.5665175,
                  "Y": 0.3789011
                },
                {
                  "X": 0.45205012,
                  "Y": 0.3784284
                }
              ]
            },
            "Id": "3b7bf82e-cdfc-40b0-8cec-a125cf841462",
            "Relationships": [
              {
                "Ids": [
                  "275740ac-87e0-47e8-8678-f98c79616071",
                  "97c5cb63-5beb-4d6c-81ed-14ecc322909f"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "10 12.79"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.75601,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015485014,
                "Left": 0.6799429,
                "Top": 0.3610776,
                "Width": 0.066833034
              },
              "Polygon": [
                {
                  "X": 0.6804465,
                  "Y": 0.3610776
                },
                {
                  "X": 0.746776,
                  "Y": 0.36134434
                },
                {
                  "X": 0.74627167,
                  "Y": 0.37656263
                },
                {
                  "X": 0.6799429,
                  "Y": 0.37628993
                }
              ]
            },
            "Id": "322d3179-d7fb-4d59-bd85-b34e1d474b14",
            "Relationships": [
              {
                "Ids": [
                  "89b0004d-f81d-42b9-b4b9-7acf5695e067"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "12.79"
          },
          {
            "BlockType": "LINE",
            "Confidence": 98.299194,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016642123,
                "Left": 0.22681804,
                "Top": 0.37765265,
                "Width": 0.18015796
              },
              "Polygon": [
                {
                  "X": 0.22734043,
                  "Y": 0.37765265
                },
                {
                  "X": 0.40697598,
                  "Y": 0.3783946
                },
                {
                  "X": 0.4064517,
                  "Y": 0.39429477
                },
                {
                  "X": 0.22681804,
                  "Y": 0.3935359
                }
              ]
            },
            "Id": "3e75a2c0-0fcc-47fd-b13c-a8110fd8c0ec",
            "Relationships": [
              {
                "Ids": [
                  "bd0914a6-0a6c-4be6-b6d6-42598c9a438c",
                  "02740f62-c84a-441d-8838-1bb04b95543c",
                  "f476a3ba-689a-49e0-abeb-22beabd449f6"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "CPN GET ITM20%"
          },
          {
            "BlockType": "LINE",
            "Confidence": 95.780396,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014642005,
                "Left": 0.4759855,
                "Top": 0.37922654,
                "Width": 0.06355699
              },
              "Polygon": [
                {
                  "X": 0.47645998,
                  "Y": 0.37922654
                },
                {
                  "X": 0.5395425,
                  "Y": 0.3794873
                },
                {
                  "X": 0.5390674,
                  "Y": 0.39386857
                },
                {
                  "X": 0.4759855,
                  "Y": 0.39360243
                }
              ]
            },
            "Id": "f310a91a-fecf-4cec-8652-45ff6325503e",
            "Relationships": [
              {
                "Ids": [
                  "b140121c-f6f2-45a5-a1dd-8fa07f795a80"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "3.20-"
          },
          {
            "BlockType": "LINE",
            "Confidence": 76.76293,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015635224,
                "Left": 0.3772232,
                "Top": 0.39452586,
                "Width": 0.13487689
              },
              "Polygon": [
                {
                  "X": 0.37771943,
                  "Y": 0.39452586
                },
                {
                  "X": 0.5121001,
                  "Y": 0.39509386
                },
                {
                  "X": 0.5116025,
                  "Y": 0.4101611
                },
                {
                  "X": 0.3772232,
                  "Y": 0.40958112
                }
              ]
            },
            "Id": "5b4897a4-e4fb-4599-a9de-2f218a4972a6",
            "Relationships": [
              {
                "Ids": [
                  "86fcbec5-54f1-4599-8ab9-47b0bf4a7a7d",
                  "cafce452-711d-4e0b-8f83-d33de2bf492c"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "YOU SAVED $"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.904816,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015397385,
                "Left": 0.54934925,
                "Top": 0.39373937,
                "Width": 0.06848461
              },
              "Polygon": [
                {
                  "X": 0.5498483,
                  "Y": 0.39373937
                },
                {
                  "X": 0.61783385,
                  "Y": 0.39402613
                },
                {
                  "X": 0.61733407,
                  "Y": 0.40913677
                },
                {
                  "X": 0.54934925,
                  "Y": 0.40884393
                }
              ]
            },
            "Id": "5597e6e2-1c4f-427c-8c05-64754e10a55e",
            "Relationships": [
              {
                "Ids": [
                  "43ef554b-5c47-480b-a18b-f42530b4f7ba"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "23.19"
          },
          {
            "BlockType": "LINE",
            "Confidence": 85.28238,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018108394,
                "Left": 0.12175347,
                "Top": 0.40551037,
                "Width": 0.23164643
              },
              "Polygon": [
                {
                  "X": 0.12231475,
                  "Y": 0.40551037
                },
                {
                  "X": 0.3533999,
                  "Y": 0.40650365
                },
                {
                  "X": 0.35283598,
                  "Y": 0.42361876
                },
                {
                  "X": 0.12175347,
                  "Y": 0.42260206
                }
              ]
            },
            "Id": "ffab194e-5506-46d3-b6e4-a86f66e1e951",
            "Relationships": [
              {
                "Ids": [
                  "9b64043a-4bda-4615-8e44-0db8058854fb",
                  "22836ed2-d521-4f7f-8008-eb578698dcb7"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Coupon(s) Applied"
          },
          {
            "BlockType": "LINE",
            "Confidence": 95.47233,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017784195,
                "Left": 0.14754468,
                "Top": 0.41963613,
                "Width": 0.35664248
              },
              "Polygon": [
                {
                  "X": 0.14807653,
                  "Y": 0.41963613
                },
                {
                  "X": 0.50418717,
                  "Y": 0.42119646
                },
                {
                  "X": 0.50365144,
                  "Y": 0.43742034
                },
                {
                  "X": 0.14754468,
                  "Y": 0.4358258
                }
              ]
            },
            "Id": "786aa80d-4e9f-4fb7-ad0c-710dc2265b04",
            "Relationships": [
              {
                "Ids": [
                  "1222fb41-4c5d-4056-9cef-8f23d9d9608a",
                  "d5a9e60d-fac7-4900-bd73-d90db2855ca4",
                  "225cca9f-92a6-4917-9a53-d1674446df31",
                  "c4855527-a861-4c1a-b73a-beeefa7275ea"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "400100949528 CPN GET ITM20%"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.738754,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015416053,
                "Left": 0.28961253,
                "Top": 0.43507823,
                "Width": 0.10022858
              },
              "Polygon": [
                {
                  "X": 0.29010504,
                  "Y": 0.43507823
                },
                {
                  "X": 0.3898411,
                  "Y": 0.435524
                },
                {
                  "X": 0.38934758,
                  "Y": 0.4504943
                },
                {
                  "X": 0.28961253,
                  "Y": 0.45003965
                }
              ]
            },
            "Id": "6f0ef31f-c8cb-48cd-a65b-b33c3cdfa0d0",
            "Relationships": [
              {
                "Ids": [
                  "34766663-8133-4635-8cf2-a35c3a7285de"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "SUBTOTAL"
          },
          {
            "BlockType": "LINE",
            "Confidence": 98.88723,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0154617885,
                "Left": 0.5111842,
                "Top": 0.4356041,
                "Width": 0.06752795
              },
              "Polygon": [
                {
                  "X": 0.5116848,
                  "Y": 0.4356041
                },
                {
                  "X": 0.57871217,
                  "Y": 0.4359035
                },
                {
                  "X": 0.57821095,
                  "Y": 0.45106587
                },
                {
                  "X": 0.5111842,
                  "Y": 0.45076045
                }
              ]
            },
            "Id": "0cf1fce8-e93b-467d-bbd6-288345048259",
            "Relationships": [
              {
                "Ids": [
                  "2fe59bba-772a-4531-878f-616267977f6c"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "24.77"
          },
          {
            "BlockType": "LINE",
            "Confidence": 86.724304,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0157122,
                "Left": 0.21009046,
                "Top": 0.44859707,
                "Width": 0.12922148
              },
              "Polygon": [
                {
                  "X": 0.21058746,
                  "Y": 0.44859707
                },
                {
                  "X": 0.33931193,
                  "Y": 0.44918302
                },
                {
                  "X": 0.33881363,
                  "Y": 0.46430928
                },
                {
                  "X": 0.21009046,
                  "Y": 0.4637118
                }
              ]
            },
            "Id": "7f067cc2-1c8b-42fd-a65d-d5608aa2c284",
            "Relationships": [
              {
                "Ids": [
                  "6381051e-2bac-4bb5-bb63-de7f71596ffd",
                  "4942dbe1-d371-42eb-9bf5-6bb938a3517e",
                  "e90ed663-4d3c-41a4-8e43-12d4d63ac969"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "PIF 1.00 %"
          },
          {
            "BlockType": "LINE",
            "Confidence": 63.02355,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013023372,
                "Left": 0.54264784,
                "Top": 0.45162806,
                "Width": 0.03513037
              },
              "Polygon": [
                {
                  "X": 0.5430728,
                  "Y": 0.45162806
                },
                {
                  "X": 0.5777782,
                  "Y": 0.45178634
                },
                {
                  "X": 0.57735294,
                  "Y": 0.4646514
                },
                {
                  "X": 0.54264784,
                  "Y": 0.46449047
                }
              ]
            },
            "Id": "ea1cc434-5beb-40c5-a678-e53090d41c65",
            "Relationships": [
              {
                "Ids": [
                  "5e69deea-ba83-495c-b43d-86d75c7d4eac"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "25"
          },
          {
            "BlockType": "LINE",
            "Confidence": 92.5605,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01568083,
                "Left": 0.20980147,
                "Top": 0.46284455,
                "Width": 0.1772441
              },
              "Polygon": [
                {
                  "X": 0.21028963,
                  "Y": 0.46284455
                },
                {
                  "X": 0.38704556,
                  "Y": 0.46366405
                },
                {
                  "X": 0.38655564,
                  "Y": 0.47852537
                },
                {
                  "X": 0.20980147,
                  "Y": 0.47769028
                }
              ]
            },
            "Id": "86015541-908b-4e87-b817-fa528e980b9c",
            "Relationships": [
              {
                "Ids": [
                  "2bb7ca27-94fd-4d36-af86-61da98142b5e",
                  "1d81d94a-1915-44f5-862e-8f6ede7f6091"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "SUBTOTAL W/PIF"
          },
          {
            "BlockType": "LINE",
            "Confidence": 95.997604,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013931713,
                "Left": 0.51215535,
                "Top": 0.46471348,
                "Width": 0.06702088
              },
              "Polygon": [
                {
                  "X": 0.5126051,
                  "Y": 0.46471348
                },
                {
                  "X": 0.57917625,
                  "Y": 0.46502233
                },
                {
                  "X": 0.5787259,
                  "Y": 0.4786452
                },
                {
                  "X": 0.51215535,
                  "Y": 0.478331
                }
              ]
            },
            "Id": "7a28cc9d-cd79-4625-b461-4521367ae421",
            "Relationships": [
              {
                "Ids": [
                  "f0cdc3e3-c5c7-493f-940e-8fa96e274148"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "25.02"
          },
          {
            "BlockType": "LINE",
            "Confidence": 95.232086,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016210036,
                "Left": 0.20817952,
                "Top": 0.47744006,
                "Width": 0.18201354
              },
              "Polygon": [
                {
                  "X": 0.20868379,
                  "Y": 0.47744006
                },
                {
                  "X": 0.39019307,
                  "Y": 0.47829735
                },
                {
                  "X": 0.3896869,
                  "Y": 0.49365008
                },
                {
                  "X": 0.20817952,
                  "Y": 0.49277627
                }
              ]
            },
            "Id": "74c5385d-b9b5-4aff-9a48-d433cba012e4",
            "Relationships": [
              {
                "Ids": [
                  "63da16f3-eb74-4b0a-b394-1605b29b3cd7",
                  "50c7724f-9036-418b-84f1-4575b85a778e",
                  "21aba29b-fba1-4440-917e-52733f389e88"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Sales Tax 6.5%"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.7047,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013707429,
                "Left": 0.524055,
                "Top": 0.47901157,
                "Width": 0.05524973
              },
              "Polygon": [
                {
                  "X": 0.52449906,
                  "Y": 0.47901157
                },
                {
                  "X": 0.5793047,
                  "Y": 0.47927043
                },
                {
                  "X": 0.57886016,
                  "Y": 0.492719
                },
                {
                  "X": 0.524055,
                  "Y": 0.49245575
                }
              ]
            },
            "Id": "65c9697e-5033-4528-8c5b-83cdfc7f4a5c",
            "Relationships": [
              {
                "Ids": [
                  "96dc5f08-210e-4bbc-a3c4-2b9c512f636b"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "1.63"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.81102,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015007724,
                "Left": 0.32206234,
                "Top": 0.49233186,
                "Width": 0.06629092
              },
              "Polygon": [
                {
                  "X": 0.32254604,
                  "Y": 0.49233186
                },
                {
                  "X": 0.38835326,
                  "Y": 0.49264827
                },
                {
                  "X": 0.38786894,
                  "Y": 0.5073396
                },
                {
                  "X": 0.32206234,
                  "Y": 0.50701743
                }
              ]
            },
            "Id": "a55c834d-66fd-4389-a6f4-99832796bd0e",
            "Relationships": [
              {
                "Ids": [
                  "5bc3c123-6580-463d-bc75-e7c4c779a860"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "TOTAL"
          },
          {
            "BlockType": "LINE",
            "Confidence": 82.55806,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0149916215,
                "Left": 0.5115432,
                "Top": 0.49297485,
                "Width": 0.06800821
              },
              "Polygon": [
                {
                  "X": 0.5120274,
                  "Y": 0.49297485
                },
                {
                  "X": 0.5795514,
                  "Y": 0.4932994
                },
                {
                  "X": 0.5790666,
                  "Y": 0.50796646
                },
                {
                  "X": 0.5115432,
                  "Y": 0.507636
                }
              ]
            },
            "Id": "b138bf94-3c75-49f3-9e99-db33fadf6fe4",
            "Relationships": [
              {
                "Ids": [
                  "b15d5fc1-9f6e-4d93-b1e2-fc69dd900c18"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "26.65"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.63057,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016768375,
                "Left": 0.11347502,
                "Top": 0.510334,
                "Width": 0.18567342
              },
              "Polygon": [
                {
                  "X": 0.11399514,
                  "Y": 0.510334
                },
                {
                  "X": 0.29914844,
                  "Y": 0.5112452
                },
                {
                  "X": 0.29862633,
                  "Y": 0.5271024
                },
                {
                  "X": 0.11347502,
                  "Y": 0.52617383
                }
              ]
            },
            "Id": "e41e6dcf-0bb4-4ef6-a7c6-fed797ffac9a",
            "Relationships": [
              {
                "Ids": [
                  "000c3d92-ca49-4e96-b2e3-d7a3c1e94489",
                  "dd753112-ac06-45eb-89eb-0366c2a5befb"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "ACCOUNT NUMBER"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.37107,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014611467,
                "Left": 0.1518607,
                "Top": 0.52709156,
                "Width": 0.057538606
              },
              "Polygon": [
                {
                  "X": 0.15233119,
                  "Y": 0.52709156
                },
                {
                  "X": 0.2093993,
                  "Y": 0.527378
                },
                {
                  "X": 0.20892826,
                  "Y": 0.541703
                },
                {
                  "X": 0.1518607,
                  "Y": 0.5414117
                }
              ]
            },
            "Id": "4964345b-5ad3-435b-908d-aab385499bf6",
            "Relationships": [
              {
                "Ids": [
                  "dd83761b-b012-4b51-b289-80c4bc2da201"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Visa"
          },
          {
            "BlockType": "LINE",
            "Confidence": 95.98092,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015559327,
                "Left": 0.5106136,
                "Top": 0.5266225,
                "Width": 0.06945804
              },
              "Polygon": [
                {
                  "X": 0.51111585,
                  "Y": 0.5266225
                },
                {
                  "X": 0.5800716,
                  "Y": 0.52696764
                },
                {
                  "X": 0.5795687,
                  "Y": 0.5421818
                },
                {
                  "X": 0.5106136,
                  "Y": 0.54183036
                }
              ]
            },
            "Id": "fe9fa440-320e-4272-ae68-0c843cbd652f",
            "Relationships": [
              {
                "Ids": [
                  "f2a7a2e9-684f-445d-9662-8e86e4d5fee1"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "26.65"
          },
          {
            "BlockType": "LINE",
            "Confidence": 96.91468,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.019157767,
                "Left": 0.1491293,
                "Top": 0.53881633,
                "Width": 0.3639048
              },
              "Polygon": [
                {
                  "X": 0.14969674,
                  "Y": 0.53881633
                },
                {
                  "X": 0.5130341,
                  "Y": 0.54066545
                },
                {
                  "X": 0.51246244,
                  "Y": 0.5579741
                },
                {
                  "X": 0.1491293,
                  "Y": 0.5560877
                }
              ]
            },
            "Id": "48473399-452c-4bc1-912f-70821b29b9f4",
            "Relationships": [
              {
                "Ids": [
                  "edafaf31-b69b-43aa-b454-fd73b6d79bfa",
                  "96913de3-e024-47d9-9dc0-00e9db1f99ec",
                  "f46e8a21-6eb3-44c8-a3f0-19ebcb78c12b",
                  "3eaa6ecc-980a-40a1-8928-780c879b904e"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "APPROVAL 00751C CHIP ONLINE"
          },
          {
            "BlockType": "LINE",
            "Confidence": 97.41439,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018549133,
                "Left": 0.16235839,
                "Top": 0.5542117,
                "Width": 0.3883579
              },
              "Polygon": [
                {
                  "X": 0.16290066,
                  "Y": 0.5542117
                },
                {
                  "X": 0.5507163,
                  "Y": 0.5562207
                },
                {
                  "X": 0.5501697,
                  "Y": 0.5727608
                },
                {
                  "X": 0.16235839,
                  "Y": 0.5707138
                }
              ]
            },
            "Id": "70b064da-afce-41e4-8292-a648bbfc80ea",
            "Relationships": [
              {
                "Ids": [
                  "8d3acceb-dd6b-4850-838f-d13ae2b95c62",
                  "e9a3f929-a77f-4646-abd7-2e6d2c61d3d1",
                  "0ad4221e-da4c-4cb9-8479-b3468a8f1078",
                  "408c2d79-c8a4-42c3-a0f9-af8f81715593"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Application Label: VISA CREDIT"
          },
          {
            "BlockType": "LINE",
            "Confidence": 94.482445,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016715534,
                "Left": 0.16144538,
                "Top": 0.57009387,
                "Width": 0.2481513
              },
              "Polygon": [
                {
                  "X": 0.16195099,
                  "Y": 0.57009387
                },
                {
                  "X": 0.40959668,
                  "Y": 0.57140017
                },
                {
                  "X": 0.4090885,
                  "Y": 0.58680946
                },
                {
                  "X": 0.16144538,
                  "Y": 0.5854806
                }
              ]
            },
            "Id": "57f3bcf8-e7e6-4cca-b9fe-b9f218236b00",
            "Relationships": [
              {
                "Ids": [
                  "824aeb91-e330-4b9f-990e-56946ff592d3",
                  "6f8772c4-e365-4bb4-957e-b347d03f3aec"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "AID: A0000000031010"
          },
          {
            "BlockType": "LINE",
            "Confidence": 90.61434,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015533249,
                "Left": 0.16102281,
                "Top": 0.58616924,
                "Width": 0.19773458
              },
              "Polygon": [
                {
                  "X": 0.16149788,
                  "Y": 0.58616924
                },
                {
                  "X": 0.3587574,
                  "Y": 0.5872286
                },
                {
                  "X": 0.35828042,
                  "Y": 0.6017025
                },
                {
                  "X": 0.16102281,
                  "Y": 0.6006263
                }
              ]
            },
            "Id": "ba21923f-58e5-4836-a0ce-8d7248380997",
            "Relationships": [
              {
                "Ids": [
                  "a1b1c0bf-ef74-4480-addb-02d406e95f7d",
                  "721053f8-d26d-4c79-beef-3ab14a9c6baa"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "TVR: 0880008000"
          },
          {
            "BlockType": "LINE",
            "Confidence": 85.60739,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01555645,
                "Left": 0.1609792,
                "Top": 0.6024514,
                "Width": 0.11975074
              },
              "Polygon": [
                {
                  "X": 0.16146863,
                  "Y": 0.6024514
                },
                {
                  "X": 0.28072992,
                  "Y": 0.6031034
                },
                {
                  "X": 0.2802393,
                  "Y": 0.61800784
                },
                {
                  "X": 0.1609792,
                  "Y": 0.61734533
                }
              ]
            },
            "Id": "087bcf0c-938f-4c05-a177-c1dc66fe096e",
            "Relationships": [
              {
                "Ids": [
                  "85bd580c-e48d-4ba4-9e4d-1ecf1c6c839b",
                  "8a8ca841-70e5-43ad-8f4e-ad79998505e1"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "TSI: E800"
          },
          {
            "BlockType": "LINE",
            "Confidence": 91.20289,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.023563696,
                "Left": 0.16510606,
                "Top": 0.61783004,
                "Width": 0.5592226
              },
              "Polygon": [
                {
                  "X": 0.1657762,
                  "Y": 0.61783004
                },
                {
                  "X": 0.72432864,
                  "Y": 0.62093455
                },
                {
                  "X": 0.7236509,
                  "Y": 0.6413938
                },
                {
                  "X": 0.16510606,
                  "Y": 0.6382217
                }
              ]
            },
            "Id": "c26361cb-450f-4d9a-b220-71bb5cc530e0",
            "Relationships": [
              {
                "Ids": [
                  "bc30a0f9-b3f3-412a-80a5-570f3f075242",
                  "63f8325e-13f8-4435-b23e-51ad86f73922",
                  "634936aa-ae04-4a64-aed8-42f9beaeccc5",
                  "1cb11244-6020-45cb-9ec0-e8cbf4d2bbe0",
                  "c888542e-d478-4547-ac00-5b1346090266",
                  "d48cd37b-82ca-41e6-ae37-19fcede8ea8a",
                  "ba5e8762-c7b3-421e-847c-1467fba0e29a",
                  "1930000e-4651-4837-b7a3-ebe7d43fb6ae"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "This receipt expires at 60 days on 03/04/22"
          },
          {
            "BlockType": "LINE",
            "Confidence": 97.338234,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.022615297,
                "Left": 0.18844669,
                "Top": 0.6343904,
                "Width": 0.5079731
              },
              "Polygon": [
                {
                  "X": 0.18909387,
                  "Y": 0.6343904
                },
                {
                  "X": 0.6964198,
                  "Y": 0.63725966
                },
                {
                  "X": 0.69576585,
                  "Y": 0.6570057
                },
                {
                  "X": 0.18844669,
                  "Y": 0.65407723
                }
              ]
            },
            "Id": "d47b4a25-e9c1-450b-a02c-ab51a57da37b",
            "Relationships": [
              {
                "Ids": [
                  "0ccb3e42-0e9a-4d24-9a99-9a16c62dfed1",
                  "e615db66-a78e-4f2f-bef3-f670cf8dca90",
                  "7d184f33-e3ec-41fe-ba2a-6909b844ef18",
                  "b03955f8-7596-45d4-b7b2-acd7191776eb",
                  "6fc32356-2209-4a95-be2a-b42b916a5133"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Previous Michaels Rewards Balance: $0.0"
          },
          {
            "BlockType": "LINE",
            "Confidence": 85.13239,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.023975436,
                "Left": 0.15292211,
                "Top": 0.6522483,
                "Width": 0.57106364
              },
              "Polygon": [
                {
                  "X": 0.15359959,
                  "Y": 0.6522483
                },
                {
                  "X": 0.7239858,
                  "Y": 0.6555354
                },
                {
                  "X": 0.7233004,
                  "Y": 0.67622375
                },
                {
                  "X": 0.15292211,
                  "Y": 0.6728669
                }
              ]
            },
            "Id": "e64f087e-cb69-42f4-b01b-6950c022597c",
            "Relationships": [
              {
                "Ids": [
                  "1b169f8a-08fb-4fb1-820e-af7fa8f7c041",
                  "f4974966-2878-42bc-9761-93a2467db24a",
                  "30086c7d-b386-44a7-829d-b1a4d49c6259",
                  "d5490f7e-840c-4f31-9bfb-c6640e9b7dbf",
                  "9ad2901e-2a19-4b02-bef0-4f5df63f2421",
                  "cf1ae385-00d9-42cd-82ea-01ff61a63f62",
                  "05527001-d1e5-47d8-b78e-b7cd4301e3ac"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Click Buy Create Shop michaels com today!"
          },
          {
            "BlockType": "LINE",
            "Confidence": 93.32577,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.025537169,
                "Left": 0.1178321,
                "Top": 0.6697419,
                "Width": 0.64388186
              },
              "Polygon": [
                {
                  "X": 0.11854412,
                  "Y": 0.6697419
                },
                {
                  "X": 0.761714,
                  "Y": 0.67351604
                },
                {
                  "X": 0.7609926,
                  "Y": 0.6952791
                },
                {
                  "X": 0.1178321,
                  "Y": 0.6914222
                }
              ]
            },
            "Id": "31857a6b-4130-47ff-8b00-4752a18efc4b",
            "Relationships": [
              {
                "Ids": [
                  "325d3c70-df0a-460e-ba59-795214f5ecd2",
                  "854fa964-5351-4418-99a8-a2de84a9a3b4",
                  "01016d9a-7a68-469e-adb9-b09d93193fca",
                  "9965d26c-13ca-4bde-b7b2-ea83e675bab7",
                  "4cf15bd5-1884-4c29-a887-1d9ceec7d45f",
                  "498a6065-5051-4c82-a2a6-5b9fc14245c3",
                  "4a790e7c-e4ab-4137-b784-1353123004ad",
                  "a848e46b-568c-4a47-b4ae-e451cae28829"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Get Savings & Inspiration Text* SIGNUP to 273283"
          },
          {
            "BlockType": "LINE",
            "Confidence": 90.9884,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.020605624,
                "Left": 0.20066923,
                "Top": 0.68597466,
                "Width": 0.47610047
              },
              "Polygon": [
                {
                  "X": 0.20125191,
                  "Y": 0.68597466
                },
                {
                  "X": 0.6767697,
                  "Y": 0.68880945
                },
                {
                  "X": 0.6761814,
                  "Y": 0.7065803
                },
                {
                  "X": 0.20066923,
                  "Y": 0.7036955
                }
              ]
            },
            "Id": "55bb4050-e9a9-4177-93ce-e44c8f6c9615",
            "Relationships": [
              {
                "Ids": [
                  "62ba6f0e-e64c-4bf6-b7da-f21234523ace",
                  "e80b1281-efc8-4736-a57b-ad242bc6e75b",
                  "f489dd8a-7938-4909-ac4b-0d44fe4619f5",
                  "0f35c8b1-07c1-4f22-b204-cf09ba9d3e69",
                  "a4b5df56-c08c-44a3-b94e-71a3b444d96f",
                  "2a432d42-6ccc-4abd-86c6-8b2f22e44a19",
                  "83065128-fe04-4ad2-9a29-a585b7cb6eee",
                  "88ee480e-96d6-4407-bc26-fe28fed0742f"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "To Sign Up for Email & Text Messages"
          },
          {
            "BlockType": "LINE",
            "Confidence": 80.81296,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018399827,
                "Left": 0.2665825,
                "Top": 0.70122004,
                "Width": 0.3497437
              },
              "Polygon": [
                {
                  "X": 0.26711744,
                  "Y": 0.70122004
                },
                {
                  "X": 0.61632615,
                  "Y": 0.7033326
                },
                {
                  "X": 0.61578745,
                  "Y": 0.71961987
                },
                {
                  "X": 0.2665825,
                  "Y": 0.7174736
                }
              ]
            },
            "Id": "5cdd5351-7b51-43d6-9a6b-7ab178f5ed7f",
            "Relationships": [
              {
                "Ids": [
                  "61401ac1-caf1-44c5-8162-81035d7dd11b",
                  "1bca35ca-1079-4a01-ae99-05fe81445769",
                  "0aa5d288-3fc8-45bb-bb99-6f60c99ba1d0",
                  "0dcb191f-dd26-4bd9-8c1e-8aa5f8832eea",
                  "a0f96620-d531-48e6-b9ce-b58db04828ea",
                  "c46a1bf9-b3ed-4d7e-bb57-e0138d937d90"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "*Msg & Date Rates May Apply"
          },
          {
            "BlockType": "LINE",
            "Confidence": 94.70973,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017579148,
                "Left": 0.19894846,
                "Top": 0.716149,
                "Width": 0.4808727
              },
              "Polygon": [
                {
                  "X": 0.19942811,
                  "Y": 0.716149
                },
                {
                  "X": 0.67982113,
                  "Y": 0.7190989
                },
                {
                  "X": 0.6793368,
                  "Y": 0.7337281
                },
                {
                  "X": 0.19894846,
                  "Y": 0.7307366
                }
              ]
            },
            "Id": "08ee383e-05c2-43b1-96c9-690e6e955e8e",
            "Relationships": [
              {
                "Ids": [
                  "80b7bb38-f809-46fc-aa29-0d874a49d181",
                  "2b8f0af8-95a3-4a34-beb9-5be4772ffc00",
                  "0c3ffd65-ab3e-4846-97fa-bfa59d4d34f5",
                  "171138bf-a44d-4449-a84d-3777a2b2a1f7",
                  "0fc12bed-1ca0-4827-9546-8ab7a10d4c32",
                  "e3e96cc6-e86c-465e-8eff-0da20c8abd00"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "You will receive 1 autodialed message"
          },
          {
            "BlockType": "LINE",
            "Confidence": 89.80159,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.019944282,
                "Left": 0.20422523,
                "Top": 0.72921616,
                "Width": 0.46061623
              },
              "Polygon": [
                {
                  "X": 0.20478547,
                  "Y": 0.72921616
                },
                {
                  "X": 0.6648415,
                  "Y": 0.7320768
                },
                {
                  "X": 0.66427594,
                  "Y": 0.7491604
                },
                {
                  "X": 0.20422523,
                  "Y": 0.74625325
                }
              ]
            },
            "Id": "6e289678-886d-450f-86e7-f19b28e5799c",
            "Relationships": [
              {
                "Ids": [
                  "1d9ce091-86fe-4033-afac-3659b9d01374",
                  "3185d70c-17ae-44d9-a281-406942a52e80",
                  "c9bc2dcb-368d-4fb7-b0d4-d4d38299f3f5",
                  "b356121b-3b06-407b-839f-a3d36310be11",
                  "355e2a34-36ee-43b6-929b-5c79116bc414",
                  "236c7312-69a4-452b-8ec7-26b859001ec6",
                  "e03fb7d4-6af4-41b8-8c87-d339ca346b44"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "with a link to join Michaels alerts"
          },
          {
            "BlockType": "LINE",
            "Confidence": 83.89466,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017375702,
                "Left": 0.34749302,
                "Top": 0.74939,
                "Width": 0.18318309
              },
              "Polygon": [
                {
                  "X": 0.34802687,
                  "Y": 0.74939
                },
                {
                  "X": 0.5306761,
                  "Y": 0.75054663
                },
                {
                  "X": 0.5301402,
                  "Y": 0.7667657
                },
                {
                  "X": 0.34749302,
                  "Y": 0.76559156
                }
              ]
            },
            "Id": "68006f3d-04fb-47b2-bccd-f7bf1120efec",
            "Relationships": [
              {
                "Ids": [
                  "0f89a2d0-f542-4873-954f-9aabb5ed640c",
                  "3bba5cae-18ad-4a26-a665-4bac3ebbc85f"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Baron Brothers"
          },
          {
            "BlockType": "LINE",
            "Confidence": 92.36005,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015959803,
                "Left": 0.34758976,
                "Top": 0.7648174,
                "Width": 0.18343806
              },
              "Polygon": [
                {
                  "X": 0.3480764,
                  "Y": 0.7648174
                },
                {
                  "X": 0.5310278,
                  "Y": 0.7659927
                },
                {
                  "X": 0.53053933,
                  "Y": 0.7807772
                },
                {
                  "X": 0.34758976,
                  "Y": 0.77958596
                }
              ]
            },
            "Id": "dc5a16dc-cd3c-4f5e-bbf2-ee355c72e3ba",
            "Relationships": [
              {
                "Ids": [
                  "f3a6862d-799a-4d13-89de-5bdeffd6ee61",
                  "99f143d6-092e-4111-bb81-66927e99c498"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Custom Franing"
          },
          {
            "BlockType": "LINE",
            "Confidence": 96.766624,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.02339289,
                "Left": 0.13492343,
                "Top": 0.7748327,
                "Width": 0.6072453
              },
              "Polygon": [
                {
                  "X": 0.13556026,
                  "Y": 0.7748327
                },
                {
                  "X": 0.7421687,
                  "Y": 0.77877057
                },
                {
                  "X": 0.741524,
                  "Y": 0.7982256
                },
                {
                  "X": 0.13492343,
                  "Y": 0.79421794
                }
              ]
            },
            "Id": "74c62dd8-47ad-4c49-9f73-9aa99129ea87",
            "Relationships": [
              {
                "Ids": [
                  "fbd3ad71-b19c-4969-8bcd-cde3f563859a",
                  "e6b14515-4b7f-4929-ae9c-ce7077a4461c",
                  "9f1ba886-bf2f-404e-803c-6d74075edce8",
                  "5aefff8b-f19b-4425-9bae-7ddff54918cd",
                  "f71ad56e-ed34-4e0a-a5d9-ebb7b3c53128",
                  "3f639ebb-97d4-4622-8147-2ce1f04333ba",
                  "2868f44c-99fe-4a85-ba73-ab6daf3fb063",
                  "ce4c895c-fe07-410a-a603-3ab67b5715b8",
                  "47733e78-27a6-4141-a93d-1f6cb5be4c3b"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "New! Now in Over 1,200 Michaels Stores & Online"
          },
          {
            "BlockType": "LINE",
            "Confidence": 92.78623,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.023775073,
                "Left": 0.19042765,
                "Top": 0.8046961,
                "Width": 0.49146312
              },
              "Polygon": [
                {
                  "X": 0.1910998,
                  "Y": 0.8046961
                },
                {
                  "X": 0.6818908,
                  "Y": 0.8079681
                },
                {
                  "X": 0.6812119,
                  "Y": 0.8284711
                },
                {
                  "X": 0.19042765,
                  "Y": 0.82513964
                }
              ]
            },
            "Id": "3e0678ac-429f-4c4d-8009-18c0335f3f09",
            "Relationships": [
              {
                "Ids": [
                  "25256a21-fe0a-4496-ba3a-3b70099d19af",
                  "06b4f131-be81-4aa5-ae02-2139556ca5af",
                  "72823059-fd2c-451d-8c74-87e2a1782f9f",
                  "1553246b-0e00-4e94-b602-e738fde2f21f",
                  "b786d103-fac9-4d0c-9bbb-5bfc1b3f4aa6"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Now Hiring! Apply at michaels.com/jobs"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.66319,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.021281794,
                "Left": 0.21747968,
                "Top": 0.8341143,
                "Width": 0.4387235
              },
              "Polygon": [
                {
                  "X": 0.21807954,
                  "Y": 0.8341143
                },
                {
                  "X": 0.6562032,
                  "Y": 0.83711123
                },
                {
                  "X": 0.655598,
                  "Y": 0.8553961
                },
                {
                  "X": 0.21747968,
                  "Y": 0.8523518
                }
              ]
            },
            "Id": "4f1f554a-029e-47ef-92d4-58ceb57c4e2a",
            "Relationships": [
              {
                "Ids": [
                  "5f1e4906-04ba-477f-b8ef-2d3cc7311d11",
                  "548b4674-9022-40c5-880c-db76bb3e93ea",
                  "36927511-715a-43da-96f7-ea6507f25823",
                  "5ede9d08-60ca-436c-a756-3958342afcf7",
                  "a2c25ddb-1910-4c7c-9d28-90b9c8c69625",
                  "5408fb6f-6e03-403f-bc65-7c48f5d58922"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "THANK YOU FOR SHOPPING AT MICHAELS"
          },
          {
            "BlockType": "LINE",
            "Confidence": 91.851906,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018647056,
                "Left": 0.298673,
                "Top": 0.85401326,
                "Width": 0.26919088
              },
              "Polygon": [
                {
                  "X": 0.29922464,
                  "Y": 0.85401326
                },
                {
                  "X": 0.5678639,
                  "Y": 0.85588163
                },
                {
                  "X": 0.5673092,
                  "Y": 0.8726603
                },
                {
                  "X": 0.298673,
                  "Y": 0.8707652
                }
              ]
            },
            "Id": "7feb9660-c2e4-4cf3-8b26-6622584ae7ce",
            "Relationships": [
              {
                "Ids": [
                  "7199eaff-6aa5-453d-a8f5-f0451d2c25b7",
                  "345930c1-2ebb-44e8-ba63-515eff98496a",
                  "18a31233-6afe-4258-8e6c-79cdb71e70db"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Dear Valued Customer:"
          },
          {
            "BlockType": "LINE",
            "Confidence": 91.86492,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.024855504,
                "Left": 0.12009527,
                "Top": 0.86588544,
                "Width": 0.63151515
              },
              "Polygon": [
                {
                  "X": 0.120763436,
                  "Y": 0.86588544
                },
                {
                  "X": 0.7516104,
                  "Y": 0.87032217
                },
                {
                  "X": 0.7509336,
                  "Y": 0.89074093
                },
                {
                  "X": 0.12009527,
                  "Y": 0.88622797
                }
              ]
            },
            "Id": "82469eae-4852-4058-8406-deee87e9f0fc",
            "Relationships": [
              {
                "Ids": [
                  "4bc11598-89fb-4f9a-b283-4bbdf4e8b4aa",
                  "edca59bf-f528-4cfd-8a46-458ed696960c",
                  "787fb150-84fc-45d4-a51b-3252cfb6a1a9",
                  "6ac8bc51-c291-4c21-ae27-ca4e0e47fd04",
                  "26333a73-5e6e-4261-9c6e-5f0103b73a1a",
                  "aff7a959-319d-4552-9168-3d9b618196a4",
                  "a28179c2-0a2e-4bef-949c-ac49cf84a229"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Michaels return and coupon policies and available"
          },
          {
            "BlockType": "LINE",
            "Confidence": 88.75209,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.021809664,
                "Left": 0.16446713,
                "Top": 0.883185,
                "Width": 0.5276993
              },
              "Polygon": [
                {
                  "X": 0.1650585,
                  "Y": 0.883185
                },
                {
                  "X": 0.6921664,
                  "Y": 0.88694537
                },
                {
                  "X": 0.6915687,
                  "Y": 0.9049947
                },
                {
                  "X": 0.16446713,
                  "Y": 0.9011781
                }
              ]
            },
            "Id": "63f6e1bb-133a-4afe-82c3-dff46be251b6",
            "Relationships": [
              {
                "Ids": [
                  "4b32a67e-ff94-4509-a5cf-e5f13f3124f0",
                  "80b672e1-3d46-45ed-a31c-b7c136227a51",
                  "17f16524-1576-4797-96e1-4424e69c15df",
                  "318e2217-e62e-4edb-a491-fa2275d29795",
                  "ccd140f1-b893-4042-87f7-d4036fb01086",
                  "48a67235-16d3-408d-8db7-8a017ee66b11",
                  "89bdc781-5a68-442d-ac54-95779d728140",
                  "a0b6b8d4-9735-425a-9274-20ecf7c33189"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "at Michaels COM and in store at registers"
          },
          {
            "BlockType": "LINE",
            "Confidence": 89.54102,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.024387445,
                "Left": 0.12032801,
                "Top": 0.8959357,
                "Width": 0.627539
              },
              "Polygon": [
                {
                  "X": 0.12097813,
                  "Y": 0.8959357
                },
                {
                  "X": 0.747867,
                  "Y": 0.90045655
                },
                {
                  "X": 0.7472085,
                  "Y": 0.9203232
                },
                {
                  "X": 0.12032801,
                  "Y": 0.9157287
                }
              ]
            },
            "Id": "cb03544a-f3b8-427e-88f6-d829c5b13105",
            "Relationships": [
              {
                "Ids": [
                  "c29a811a-cd28-4f85-a726-5b11cb5a78a7",
                  "67038926-23d7-4ce2-82c4-d84ec1ac4de9",
                  "be2a2127-831a-44a7-9d8a-3a1b1f161046",
                  "d11b2f4a-16a7-4278-8ffd-5a373a9ad786",
                  "6c053339-5ccf-406a-8b9d-d8a4a80b0f9d",
                  "7643dd66-9720-427e-b464-7e6d495daa7d",
                  "4c9fcb83-aa03-42e9-8f55-c5c16f6e97f3",
                  "4d03bd7d-cf3e-449d-88f8-6b91526f6771"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "*** Please be advised. effective April 15th. 2021"
          },
          {
            "BlockType": "LINE",
            "Confidence": 92.61331,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.027435374,
                "Left": 0.09821656,
                "Top": 0.9099936,
                "Width": 0.6717052
              },
              "Polygon": [
                {
                  "X": 0.098953746,
                  "Y": 0.9099936
                },
                {
                  "X": 0.7699217,
                  "Y": 0.914889
                },
                {
                  "X": 0.76917446,
                  "Y": 0.93742895
                },
                {
                  "X": 0.09821656,
                  "Y": 0.93244416
                }
              ]
            },
            "Id": "73626b59-3e04-4255-bb27-3b1e377b7568",
            "Relationships": [
              {
                "Ids": [
                  "8aa4eba9-6aa6-4f09-94da-5e0aef110251",
                  "d00cefb9-c73e-4bb3-8cb7-3765c140f9e1",
                  "6b9a2de0-a2b4-4329-b770-9246f3c5659e",
                  "469807e2-04ec-4841-aba2-478ec44ef7df",
                  "1287fad6-db8a-4f38-bb1b-1d70ca2436bb",
                  "ff70da2c-6041-46f1-9e34-d62245bc5697",
                  "0ef7c948-283a-4d38-aeb9-1710340bbb8e",
                  "879ae49f-7c66-4099-875a-39e7142410c0",
                  "fcbf3910-ecef-49a5-88c1-4f0d2365ed75",
                  "1ef3037f-1027-4ae9-b55c-2c1a836363c1"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Michaels will be moving from a 180 day return policy"
          },
          {
            "BlockType": "LINE",
            "Confidence": 92.74975,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.026567476,
                "Left": 0.09939131,
                "Top": 0.92504287,
                "Width": 0.6596342
              },
              "Polygon": [
                {
                  "X": 0.10010113,
                  "Y": 0.92504287
                },
                {
                  "X": 0.75902545,
                  "Y": 0.9299092
                },
                {
                  "X": 0.7583061,
                  "Y": 0.9516103
                },
                {
                  "X": 0.09939131,
                  "Y": 0.9466594
                }
              ]
            },
            "Id": "cff60520-ce36-4478-a59a-0777d9241bdd",
            "Relationships": [
              {
                "Ids": [
                  "9f251528-dd18-4a24-b4e9-6e3ae13d6c63",
                  "91b96ec5-d9b6-4367-9324-a23b142bea26",
                  "52535c19-5232-4f15-ae75-1d1fac825e67",
                  "e1bacdff-c199-4d51-948b-1d3701d286e3",
                  "72ce001b-1a4a-4784-a515-bb546323d1e8",
                  "6b53904d-c14c-4e9c-81dc-a050866ac522",
                  "4c68d767-0e16-4823-a147-0fa1bc5ffb18",
                  "b963e3ee-76eb-401b-a1fb-ac8ca60ac6dd",
                  "0fab1758-c52b-49fe-b438-1df19724cf61",
                  "4c044393-4c0e-4c0e-bfa8-c193469cc948",
                  "27c593f6-f9e8-4b49-b6a3-ff87d9cd3a85"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "to a 60 day return policy from the date of purchase"
          },
          {
            "BlockType": "LINE",
            "Confidence": 95.73319,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.025829913,
                "Left": 0.109374374,
                "Top": 0.9394749,
                "Width": 0.63743514
              },
              "Polygon": [
                {
                  "X": 0.11006384,
                  "Y": 0.9394749
                },
                {
                  "X": 0.74680954,
                  "Y": 0.94423175
                },
                {
                  "X": 0.7461111,
                  "Y": 0.9653048
                },
                {
                  "X": 0.109374374,
                  "Y": 0.96046853
                }
              ]
            },
            "Id": "28dc7580-d92f-485a-b488-fc5ddd2370ac",
            "Relationships": [
              {
                "Ids": [
                  "883591b3-376e-46aa-9ae3-ad5d48b393b9",
                  "1daabd12-3551-4db4-ab30-61344cccab11",
                  "17ba67f9-4101-4e0d-96f8-7f23afacb50e",
                  "21635901-0ce7-4f23-b2b7-100d19d84744",
                  "b6df18c5-354b-48fe-bf68-07ef1f47dbb4",
                  "598f136b-226b-4337-9855-00462374dcdb",
                  "b5ad6bc8-e286-4e92-8e6d-62e06c6a542f",
                  "1da2823c-c7c5-4ffc-a8de-bb5f8ba2e236"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "Please see a store associate for more information"
          },
          {
            "BlockType": "LINE",
            "Confidence": 99.297615,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.020720493,
                "Left": 0.38001359,
                "Top": 0.9683744,
                "Width": 0.18040304
              },
              "Polygon": [
                {
                  "X": 0.38065085,
                  "Y": 0.9683744
                },
                {
                  "X": 0.5604166,
                  "Y": 0.969746
                },
                {
                  "X": 0.559777,
                  "Y": 0.98909485
                },
                {
                  "X": 0.38001359,
                  "Y": 0.98770267
                }
              ]
            },
            "Id": "82ed0a28-73c2-4355-b77c-ac9504f5c921",
            "Relationships": [
              {
                "Ids": [
                  "ca91f92d-a2e3-4bc2-828a-2238c48eb50a",
                  "087a26bf-289e-4f81-b9ba-446f723efc46"
                ],
                "Type": "CHILD"
              }
            ],
            "Text": "1/04/22 13:03"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.35089,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.066532075,
                "Left": 0.27903962,
                "Top": 0.025094835,
                "Width": 0.37162614
              },
              "Polygon": [
                {
                  "X": 0.2811996,
                  "Y": 0.025094835
                },
                {
                  "X": 0.65066576,
                  "Y": 0.02584732
                },
                {
                  "X": 0.6484895,
                  "Y": 0.09162691
                },
                {
                  "X": 0.27903962,
                  "Y": 0.09073055
                }
              ]
            },
            "Id": "67e23304-4ea9-4077-8cb3-2c477bf5858b",
            "Page": 1,
            "Text": "Michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.95,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.020094993,
                "Left": 0.34572834,
                "Top": 0.104462184,
                "Width": 0.099442974
              },
              "Polygon": [
                {
                  "X": 0.34638175,
                  "Y": 0.104462184
                },
                {
                  "X": 0.44517133,
                  "Y": 0.10470983
                },
                {
                  "X": 0.4445166,
                  "Y": 0.12455718
                },
                {
                  "X": 0.34572834,
                  "Y": 0.124297924
                }
              ]
            },
            "Id": "63306dca-7a1d-4363-993b-9f867b49088f",
            "Page": 1,
            "Text": "Made",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.975876,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.02448659,
                "Left": 0.45587498,
                "Top": 0.104042515,
                "Width": 0.044249285
              },
              "Polygon": [
                {
                  "X": 0.4566791,
                  "Y": 0.104042515
                },
                {
                  "X": 0.5001243,
                  "Y": 0.10415124
                },
                {
                  "X": 0.49931946,
                  "Y": 0.1285291
                },
                {
                  "X": 0.45587498,
                  "Y": 0.12841411
                }
              ]
            },
            "Id": "a5cbac67-046a-43e2-a212-fc5e916310b4",
            "Page": 1,
            "Text": "by",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 72.61174,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.024529167,
                "Left": 0.5098391,
                "Top": 0.10321453,
                "Width": 0.07770625
              },
              "Polygon": [
                {
                  "X": 0.51064235,
                  "Y": 0.10321453
                },
                {
                  "X": 0.5875454,
                  "Y": 0.10340655
                },
                {
                  "X": 0.5867409,
                  "Y": 0.12774369
                },
                {
                  "X": 0.5098391,
                  "Y": 0.12754059
                }
              ]
            },
            "Id": "6203258b-a91e-4e59-9eef-d76abbe122e4",
            "Page": 1,
            "Text": "you",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.478874,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016262397,
                "Left": 0.23815507,
                "Top": 0.13112278,
                "Width": 0.10648912
              },
              "Polygon": [
                {
                  "X": 0.23868032,
                  "Y": 0.13112278
                },
                {
                  "X": 0.3446442,
                  "Y": 0.13140535
                },
                {
                  "X": 0.34411782,
                  "Y": 0.14738518
                },
                {
                  "X": 0.23815507,
                  "Y": 0.14709258
                }
              ]
            },
            "Id": "d27420a0-78d2-4258-8568-7bcee7a3872a",
            "Page": 1,
            "Text": "MICHAELS",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.410095,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01568785,
                "Left": 0.35404164,
                "Top": 0.13251,
                "Width": 0.062285263
              },
              "Polygon": [
                {
                  "X": 0.35455284,
                  "Y": 0.13251
                },
                {
                  "X": 0.41632688,
                  "Y": 0.13267513
                },
                {
                  "X": 0.41581503,
                  "Y": 0.14819786
                },
                {
                  "X": 0.35404164,
                  "Y": 0.14802706
                }
              ]
            },
            "Id": "f899b08c-713c-4d49-8ed1-1c77b4388ec3",
            "Page": 1,
            "Text": "STORE",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.75359,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01437471,
                "Left": 0.42867315,
                "Top": 0.13657686,
                "Width": 0.065034576
              },
              "Polygon": [
                {
                  "X": 0.42914134,
                  "Y": 0.13657686
                },
                {
                  "X": 0.49370775,
                  "Y": 0.13675092
                },
                {
                  "X": 0.49323896,
                  "Y": 0.15095156
                },
                {
                  "X": 0.42867315,
                  "Y": 0.15077206
                }
              ]
            },
            "Id": "e1a99d1d-e673-409c-b5d5-05c1bc81d7b5",
            "Page": 1,
            "Text": " #9010",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.16695,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016784983,
                "Left": 0.5133619,
                "Top": 0.1330777,
                "Width": 0.06685625
              },
              "Polygon": [
                {
                  "X": 0.5139101,
                  "Y": 0.1330777
                },
                {
                  "X": 0.58021814,
                  "Y": 0.13325499
                },
                {
                  "X": 0.57966924,
                  "Y": 0.14986268
                },
                {
                  "X": 0.5133619,
                  "Y": 0.14967887
                }
              ]
            },
            "Id": "a560b0b3-5f42-4461-95ba-0564b734925b",
            "Page": 1,
            "Text": "(386)",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 68.338875,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.021261526,
                "Left": 0.589254,
                "Top": 0.1254745,
                "Width": 0.103992455
              },
              "Polygon": [
                {
                  "X": 0.58994746,
                  "Y": 0.1254745
                },
                {
                  "X": 0.6932465,
                  "Y": 0.12574592
                },
                {
                  "X": 0.69255155,
                  "Y": 0.14673603
                },
                {
                  "X": 0.589254,
                  "Y": 0.14645177
                }
              ]
            },
            "Id": "7e8db4fd-6d52-440d-9580-b6b3516135f3",
            "Page": 1,
            "Text": "767-7495",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 87.90614,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01636611,
                "Left": 0.3301174,
                "Top": 0.14792603,
                "Width": 0.10670652
              },
              "Polygon": [
                {
                  "X": 0.3306464,
                  "Y": 0.14792603
                },
                {
                  "X": 0.43682393,
                  "Y": 0.14821959
                },
                {
                  "X": 0.43629378,
                  "Y": 0.16429214
                },
                {
                  "X": 0.3301174,
                  "Y": 0.16398847
                }
              ]
            },
            "Id": "8c177e41-a642-418f-a010-4a0970b5195b",
            "Page": 1,
            "Text": "MICHAELS",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.815056,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014897752,
                "Left": 0.44431984,
                "Top": 0.1508113,
                "Width": 0.064303204
              },
              "Polygon": [
                {
                  "X": 0.44480526,
                  "Y": 0.1508113
                },
                {
                  "X": 0.50862306,
                  "Y": 0.15098871
                },
                {
                  "X": 0.508137,
                  "Y": 0.16570905
                },
                {
                  "X": 0.44431984,
                  "Y": 0.16552608
                }
              ]
            },
            "Id": "e92faa69-8be2-4ec0-8c9f-39e7431e11ed",
            "Page": 1,
            "Text": "STORE",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 84.73109,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016408399,
                "Left": 0.51975423,
                "Top": 0.14776957,
                "Width": 0.06557272
              },
              "Polygon": [
                {
                  "X": 0.52028996,
                  "Y": 0.14776957
                },
                {
                  "X": 0.5853269,
                  "Y": 0.14794913
                },
                {
                  "X": 0.58479047,
                  "Y": 0.16417797
                },
                {
                  "X": 0.51975423,
                  "Y": 0.16399217
                }
              ]
            },
            "Id": "3fffd05c-2d78-4531-877b-9f65bf0d80d1",
            "Page": 1,
            "Text": " #9010",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 67.05735,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014189579,
                "Left": 0.32033384,
                "Top": 0.16017283,
                "Width": 0.055129413
              },
              "Polygon": [
                {
                  "X": 0.32079583,
                  "Y": 0.16017283
                },
                {
                  "X": 0.37546325,
                  "Y": 0.16032796
                },
                {
                  "X": 0.37500075,
                  "Y": 0.1743624
                },
                {
                  "X": 0.32033384,
                  "Y": 0.17420274
                }
              ]
            },
            "Id": "40482d00-1bc8-42ec-a357-33beec61ba9c",
            "Page": 1,
            "Text": "5507",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 79.01972,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013673142,
                "Left": 0.38385212,
                "Top": 0.16287959,
                "Width": 0.015832856
              },
              "Polygon": [
                {
                  "X": 0.3843013,
                  "Y": 0.16287959
                },
                {
                  "X": 0.39968497,
                  "Y": 0.16292347
                },
                {
                  "X": 0.39923567,
                  "Y": 0.17655273
                },
                {
                  "X": 0.38385212,
                  "Y": 0.1765076
                }
              ]
            },
            "Id": "123d99a7-ef99-4812-b968-7af05ded094c",
            "Page": 1,
            "Text": "S",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 70.98205,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015057063,
                "Left": 0.405024,
                "Top": 0.16434674,
                "Width": 0.0575431
              },
              "Polygon": [
                {
                  "X": 0.40551487,
                  "Y": 0.16434674
                },
                {
                  "X": 0.4625671,
                  "Y": 0.16450997
                },
                {
                  "X": 0.46207562,
                  "Y": 0.17940381
                },
                {
                  "X": 0.405024,
                  "Y": 0.17923555
                }
              ]
            },
            "Id": "2575c6fa-1269-4863-b771-9961c0427a63",
            "Page": 1,
            "Text": "WILL",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 91.634285,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01555617,
                "Left": 0.45749068,
                "Top": 0.16418722,
                "Width": 0.07535197
              },
              "Polygon": [
                {
                  "X": 0.45799667,
                  "Y": 0.16418722
                },
                {
                  "X": 0.53284264,
                  "Y": 0.16440122
                },
                {
                  "X": 0.5323359,
                  "Y": 0.1797434
                },
                {
                  "X": 0.45749068,
                  "Y": 0.1795226
                }
              ]
            },
            "Id": "4cb5d9ab-9417-4131-a9ff-be9aceae83ee",
            "Page": 1,
            "Text": "IAMSON",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.37587,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017170556,
                "Left": 0.5419287,
                "Top": 0.15968561,
                "Width": 0.052361093
              },
              "Polygon": [
                {
                  "X": 0.5424909,
                  "Y": 0.15968561
                },
                {
                  "X": 0.5942898,
                  "Y": 0.15983225
                },
                {
                  "X": 0.593727,
                  "Y": 0.17685618
                },
                {
                  "X": 0.5419287,
                  "Y": 0.1767043
                }
              ]
            },
            "Id": "8b4073f0-a834-45cd-9eca-ad0916a41de9",
            "Page": 1,
            "Text": "BLVD",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.64569,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0145497,
                "Left": 0.31846523,
                "Top": 0.17407326,
                "Width": 0.05490181
              },
              "Polygon": [
                {
                  "X": 0.31893897,
                  "Y": 0.17407326
                },
                {
                  "X": 0.37336704,
                  "Y": 0.1742322
                },
                {
                  "X": 0.3728928,
                  "Y": 0.18862295
                },
                {
                  "X": 0.31846523,
                  "Y": 0.18845938
                }
              ]
            },
            "Id": "a00b601f-c856-4598-aefa-a148240d0c42",
            "Page": 1,
            "Text": "PORT",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 51.43527,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017832007,
                "Left": 0.3813679,
                "Top": 0.17687562,
                "Width": 0.09493341
              },
              "Polygon": [
                {
                  "X": 0.38194618,
                  "Y": 0.17687562
                },
                {
                  "X": 0.4763013,
                  "Y": 0.17715262
                },
                {
                  "X": 0.47572193,
                  "Y": 0.19470763
                },
                {
                  "X": 0.3813679,
                  "Y": 0.19442081
                }
              ]
            },
            "Id": "3c597335-c575-4cb8-8b88-42a2e4676bf1",
            "Page": 1,
            "Text": "ORGANCE",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 90.31573,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0132947005,
                "Left": 0.49151897,
                "Top": 0.17896104,
                "Width": 0.03215951
              },
              "Polygon": [
                {
                  "X": 0.49195468,
                  "Y": 0.17896104
                },
                {
                  "X": 0.5236785,
                  "Y": 0.17905451
                },
                {
                  "X": 0.5232425,
                  "Y": 0.19225575
                },
                {
                  "X": 0.49151897,
                  "Y": 0.1921598
                }
              ]
            },
            "Id": "ad92a06e-478e-4206-95cc-57ad25ca9bd8",
            "Page": 1,
            "Text": "FL",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 13.055383,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016256897,
                "Left": 0.53238994,
                "Top": 0.17331472,
                "Width": 0.05939098
              },
              "Polygon": [
                {
                  "X": 0.5329211,
                  "Y": 0.17331472
                },
                {
                  "X": 0.5917809,
                  "Y": 0.17348611
                },
                {
                  "X": 0.59124917,
                  "Y": 0.18957162
                },
                {
                  "X": 0.53238994,
                  "Y": 0.18939461
                }
              ]
            },
            "Id": "2dd205fd-b2b2-455e-a8bd-749376a227b1",
            "Page": 1,
            "Text": "0328",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.2334,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017968152,
                "Left": 0.2673979,
                "Top": 0.18523951,
                "Width": 0.094420165
              },
              "Polygon": [
                {
                  "X": 0.26797962,
                  "Y": 0.18523951
                },
                {
                  "X": 0.36181808,
                  "Y": 0.18551983
                },
                {
                  "X": 0.3612353,
                  "Y": 0.20320766
                },
                {
                  "X": 0.2673979,
                  "Y": 0.20291749
                }
              ]
            },
            "Id": "883b7cd8-6644-4d07-ae16-7be3d58dbbf0",
            "Page": 1,
            "Text": "Rewards",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.210236,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018122543,
                "Left": 0.36883757,
                "Top": 0.18936704,
                "Width": 0.08411405
              },
              "Polygon": [
                {
                  "X": 0.36942616,
                  "Y": 0.18936704
                },
                {
                  "X": 0.4529516,
                  "Y": 0.18961845
                },
                {
                  "X": 0.45236197,
                  "Y": 0.20748958
                },
                {
                  "X": 0.36883757,
                  "Y": 0.20722933
                }
              ]
            },
            "Id": "dad38341-6710-43c1-bb3d-b351152035eb",
            "Page": 1,
            "Text": "Number",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 28.827736,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.019078357,
                "Left": 0.46791852,
                "Top": 0.18982321,
                "Width": 0.0721316
              },
              "Polygon": [
                {
                  "X": 0.46854073,
                  "Y": 0.18982321
                },
                {
                  "X": 0.5400501,
                  "Y": 0.19003852
                },
                {
                  "X": 0.539427,
                  "Y": 0.20890157
                },
                {
                  "X": 0.46791852,
                  "Y": 0.20867826
                }
              ]
            },
            "Id": "c977b83f-7199-435c-81e9-e9d0e9f2588b",
            "Page": 1,
            "Text": "CARROT",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.486755,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01540158,
                "Left": 0.16688687,
                "Top": 0.27234024,
                "Width": 0.0897025
              },
              "Polygon": [
                {
                  "X": 0.16738239,
                  "Y": 0.27234024
                },
                {
                  "X": 0.25658935,
                  "Y": 0.272653
                },
                {
                  "X": 0.25609294,
                  "Y": 0.2877418
                },
                {
                  "X": 0.16688687,
                  "Y": 0.28742105
                }
              ]
            },
            "Id": "46034d61-d5b3-4987-8af2-3f8861f17396",
            "Page": 1,
            "Text": "4033602",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.826,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015315034,
                "Left": 0.2917882,
                "Top": 0.2745372,
                "Width": 0.052604154
              },
              "Polygon": [
                {
                  "X": 0.29228616,
                  "Y": 0.2745372
                },
                {
                  "X": 0.34439236,
                  "Y": 0.27472046
                },
                {
                  "X": 0.3438939,
                  "Y": 0.28985226
                },
                {
                  "X": 0.2917882,
                  "Y": 0.28966433
                }
              ]
            },
            "Id": "cd3049f1-8db3-491a-9e27-6092bdd8d2ac",
            "Page": 1,
            "Text": "SALE",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 96.38991,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015842833,
                "Left": 0.41661885,
                "Top": 0.2784465,
                "Width": 0.05457047
              },
              "Polygon": [
                {
                  "X": 0.41713485,
                  "Y": 0.2784465
                },
                {
                  "X": 0.47118932,
                  "Y": 0.2786377
                },
                {
                  "X": 0.4706728,
                  "Y": 0.29428932
                },
                {
                  "X": 0.41661885,
                  "Y": 0.2940931
                }
              ]
            },
            "Id": "dc20095e-e17a-4ff8-86b3-1b4a2b98ebf7",
            "Page": 1,
            "Text": "0659",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.6922,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015166762,
                "Left": 0.47793332,
                "Top": 0.27767667,
                "Width": 0.053900745
              },
              "Polygon": [
                {
                  "X": 0.47842753,
                  "Y": 0.27767667
                },
                {
                  "X": 0.53183407,
                  "Y": 0.2778653
                },
                {
                  "X": 0.5313393,
                  "Y": 0.29284343
                },
                {
                  "X": 0.47793332,
                  "Y": 0.2926501
                }
              ]
            },
            "Id": "8c410fc1-6c99-4b14-a0f9-436fdebc5f3b",
            "Page": 1,
            "Text": "9010",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.06503,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0155895,
                "Left": 0.53775793,
                "Top": 0.27529803,
                "Width": 0.041999556
              },
              "Polygon": [
                {
                  "X": 0.538268,
                  "Y": 0.27529803
                },
                {
                  "X": 0.5797575,
                  "Y": 0.2754439
                },
                {
                  "X": 0.579247,
                  "Y": 0.29088753
                },
                {
                  "X": 0.53775793,
                  "Y": 0.29073784
                }
              ]
            },
            "Id": "159fcc1c-f3cc-46b0-9be8-f1c1d81ae13c",
            "Page": 1,
            "Text": "002",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.84677,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018050976,
                "Left": 0.60258126,
                "Top": 0.27114907,
                "Width": 0.09266121
              },
              "Polygon": [
                {
                  "X": 0.6031672,
                  "Y": 0.27114907
                },
                {
                  "X": 0.69524246,
                  "Y": 0.27147043
                },
                {
                  "X": 0.6946554,
                  "Y": 0.28920004
                },
                {
                  "X": 0.60258126,
                  "Y": 0.28886902
                }
              ]
            },
            "Id": "f33a2726-f2ea-47c4-a0b0-8756ebc686ef",
            "Page": 1,
            "Text": "1/04/22",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.76006,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.018195586,
                "Left": 0.7168045,
                "Top": 0.26736757,
                "Width": 0.06993204
              },
              "Polygon": [
                {
                  "X": 0.71739894,
                  "Y": 0.26736757
                },
                {
                  "X": 0.78673655,
                  "Y": 0.26760784
                },
                {
                  "X": 0.7861413,
                  "Y": 0.28556314
                },
                {
                  "X": 0.7168045,
                  "Y": 0.28531548
                }
              ]
            },
            "Id": "240e0ec6-8507-46fe-bca0-0b2595b94365",
            "Page": 1,
            "Text": "13:03",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 50.12367,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016036019,
                "Left": 0.12364132,
                "Top": 0.28576893,
                "Width": 0.054900803
              },
              "Polygon": [
                {
                  "X": 0.12416134,
                  "Y": 0.28576893
                },
                {
                  "X": 0.17854212,
                  "Y": 0.28596398
                },
                {
                  "X": 0.17802154,
                  "Y": 0.30180496
                },
                {
                  "X": 0.12364132,
                  "Y": 0.30160478
                }
              ]
            },
            "Id": "e45bde78-4999-4bc4-810c-c4f4e84e83c3",
            "Page": 1,
            "Text": "CNCL",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 73.72433,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01416608,
                "Left": 0.19046186,
                "Top": 0.2887505,
                "Width": 0.053828437
              },
              "Polygon": [
                {
                  "X": 0.19092102,
                  "Y": 0.2887505
                },
                {
                  "X": 0.24429029,
                  "Y": 0.2889428
                },
                {
                  "X": 0.24383062,
                  "Y": 0.3029166
                },
                {
                  "X": 0.19046186,
                  "Y": 0.30271986
                }
              ]
            },
            "Id": "4db95470-704d-4e69-9902-101597525a13",
            "Page": 1,
            "Text": "2002",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.42942,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015887199,
                "Left": 0.25334764,
                "Top": 0.28884426,
                "Width": 0.09344823
              },
              "Polygon": [
                {
                  "X": 0.253859,
                  "Y": 0.28884426
                },
                {
                  "X": 0.34679586,
                  "Y": 0.28917906
                },
                {
                  "X": 0.3462835,
                  "Y": 0.30473146
                },
                {
                  "X": 0.25334764,
                  "Y": 0.30438808
                }
              ]
            },
            "Id": "d46616be-3162-4cb7-9442-cdef40bb444f",
            "Page": 1,
            "Text": "HOLIDAY",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.526764,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015371156,
                "Left": 0.37924334,
                "Top": 0.29258725,
                "Width": 0.15386045
              },
              "Polygon": [
                {
                  "X": 0.3797312,
                  "Y": 0.29258725
                },
                {
                  "X": 0.53310376,
                  "Y": 0.29314277
                },
                {
                  "X": 0.53261435,
                  "Y": 0.3079584
                },
                {
                  "X": 0.37924334,
                  "Y": 0.30738944
                }
              ]
            },
            "Id": "2568672f-1cc0-4652-93b0-1a37464206a1",
            "Page": 1,
            "Text": "647658036793",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.88898,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015371211,
                "Left": 0.57754713,
                "Top": 0.2893452,
                "Width": 0.06596329
              },
              "Polygon": [
                {
                  "X": 0.5780472,
                  "Y": 0.2893452
                },
                {
                  "X": 0.6435104,
                  "Y": 0.28958076
                },
                {
                  "X": 0.64300966,
                  "Y": 0.3047164
                },
                {
                  "X": 0.57754713,
                  "Y": 0.30447498
                }
              ]
            },
            "Id": "edd4135e-c7a2-4018-9804-c7cebd603c38",
            "Page": 1,
            "Text": "19.99",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 51.185585,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015274781,
                "Left": 0.47886223,
                "Top": 0.3063904,
                "Width": 0.076895446
              },
              "Polygon": [
                {
                  "X": 0.47935686,
                  "Y": 0.3063904
                },
                {
                  "X": 0.5557577,
                  "Y": 0.3066732
                },
                {
                  "X": 0.55526227,
                  "Y": 0.32166517
                },
                {
                  "X": 0.47886223,
                  "Y": 0.32137558
                }
              ]
            },
            "Id": "682cc7e1-2cef-4c8e-8e31-7bd0b0cba14c",
            "Page": 1,
            "Text": "6.00",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.76659,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017144524,
                "Left": 0.6894387,
                "Top": 0.29944292,
                "Width": 0.05767767
              },
              "Polygon": [
                {
                  "X": 0.6899992,
                  "Y": 0.29944292
                },
                {
                  "X": 0.7471164,
                  "Y": 0.29965174
                },
                {
                  "X": 0.7465552,
                  "Y": 0.31658745
                },
                {
                  "X": 0.6894387,
                  "Y": 0.3163729
                }
              ]
            },
            "Id": "e1b07e60-6735-4f77-b305-ba7d1c2bcb9c",
            "Page": 1,
            "Text": "6.00",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.73283,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015874797,
                "Left": 0.7685184,
                "Top": 0.29718518,
                "Width": 0.017361006
              },
              "Polygon": [
                {
                  "X": 0.7690425,
                  "Y": 0.29718518
                },
                {
                  "X": 0.7858794,
                  "Y": 0.2972465
                },
                {
                  "X": 0.78535515,
                  "Y": 0.31306
                },
                {
                  "X": 0.7685184,
                  "Y": 0.3129971
                }
              ]
            },
            "Id": "18fc26d6-4d10-4c67-a5f9-876861c9ed7f",
            "Page": 1,
            "Text": "P",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.39969,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015387137,
                "Left": 0.123940796,
                "Top": 0.31504583,
                "Width": 0.029609218
              },
              "Polygon": [
                {
                  "X": 0.124442406,
                  "Y": 0.31504583
                },
                {
                  "X": 0.15355001,
                  "Y": 0.3151553
                },
                {
                  "X": 0.1530481,
                  "Y": 0.33043295
                },
                {
                  "X": 0.123940796,
                  "Y": 0.33032086
                }
              ]
            },
            "Id": "d3a8346a-6068-4bfc-9d09-5549b31b5de3",
            "Page": 1,
            "Text": "ST",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 87.148926,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016099652,
                "Left": 0.1654954,
                "Top": 0.31640396,
                "Width": 0.067435876
              },
              "Polygon": [
                {
                  "X": 0.16601591,
                  "Y": 0.31640396
                },
                {
                  "X": 0.23293127,
                  "Y": 0.31665608
                },
                {
                  "X": 0.23241004,
                  "Y": 0.33250362
                },
                {
                  "X": 0.1654954,
                  "Y": 0.33224517
                }
              ]
            },
            "Id": "cbdbd556-d753-41f0-9cf8-0fb10ce276e7",
            "Page": 1,
            "Text": "TREND",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.80087,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016412102,
                "Left": 0.24163349,
                "Top": 0.31755447,
                "Width": 0.06500515
              },
              "Polygon": [
                {
                  "X": 0.24216516,
                  "Y": 0.31755447
                },
                {
                  "X": 0.30663863,
                  "Y": 0.31779775
                },
                {
                  "X": 0.30610627,
                  "Y": 0.33396658
                },
                {
                  "X": 0.24163349,
                  "Y": 0.33371714
                }
              ]
            },
            "Id": "54b54b28-b572-4618-820e-6bf2b6032387",
            "Page": 1,
            "Text": "STYLE",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.75564,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015626403,
                "Left": 0.315421,
                "Top": 0.31876835,
                "Width": 0.031047955
              },
              "Polygon": [
                {
                  "X": 0.31593165,
                  "Y": 0.31876835
                },
                {
                  "X": 0.34646893,
                  "Y": 0.31888375
                },
                {
                  "X": 0.34595793,
                  "Y": 0.33439475
                },
                {
                  "X": 0.315421,
                  "Y": 0.33427656
                }
              ]
            },
            "Id": "c8d21f69-9598-49f5-9bf7-57e4d60ec1a0",
            "Page": 1,
            "Text": "PH",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.69644,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014908325,
                "Left": 0.38079435,
                "Top": 0.3207849,
                "Width": 0.15234108
              },
              "Polygon": [
                {
                  "X": 0.38126636,
                  "Y": 0.3207849
                },
                {
                  "X": 0.5331354,
                  "Y": 0.32136035
                },
                {
                  "X": 0.532662,
                  "Y": 0.3356932
                },
                {
                  "X": 0.38079435,
                  "Y": 0.33510485
                }
              ]
            },
            "Id": "2cad7d51-a29f-4489-b466-18493dce4881",
            "Page": 1,
            "Text": "192040076524",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.83552,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016229672,
                "Left": 0.5878109,
                "Top": 0.31791732,
                "Width": 0.05546808
              },
              "Polygon": [
                {
                  "X": 0.5883404,
                  "Y": 0.31791732
                },
                {
                  "X": 0.64327896,
                  "Y": 0.3181243
                },
                {
                  "X": 0.64274883,
                  "Y": 0.33414698
                },
                {
                  "X": 0.5878109,
                  "Y": 0.33393478
                }
              ]
            },
            "Id": "3446d04b-eece-4a63-b3b4-086ce5b7eb73",
            "Page": 1,
            "Text": "5.99",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 72.24644,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01617761,
                "Left": 0.45100057,
                "Top": 0.33399904,
                "Width": 0.104061194
              },
              "Polygon": [
                {
                  "X": 0.45152083,
                  "Y": 0.33399904
                },
                {
                  "X": 0.55506176,
                  "Y": 0.3343993
                },
                {
                  "X": 0.55454046,
                  "Y": 0.35017663
                },
                {
                  "X": 0.45100057,
                  "Y": 0.34976667
                }
              ]
            },
            "Id": "0af86acf-9911-4764-a8b2-2c62b72e9a83",
            "Page": 1,
            "Text": "2@2.99",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.48433,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015383451,
                "Left": 0.6886747,
                "Top": 0.33006653,
                "Width": 0.057327747
              },
              "Polygon": [
                {
                  "X": 0.68917656,
                  "Y": 0.33006653
                },
                {
                  "X": 0.74600244,
                  "Y": 0.3302846
                },
                {
                  "X": 0.7454999,
                  "Y": 0.34544998
                },
                {
                  "X": 0.6886747,
                  "Y": 0.34522685
                }
              ]
            },
            "Id": "1e785c54-01db-40bc-b9b6-99f9bcbdc97e",
            "Page": 1,
            "Text": "5.98",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.527565,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015124577,
                "Left": 0.7678011,
                "Top": 0.3286554,
                "Width": 0.017839612
              },
              "Polygon": [
                {
                  "X": 0.7683002,
                  "Y": 0.3286554
                },
                {
                  "X": 0.7856407,
                  "Y": 0.32872176
                },
                {
                  "X": 0.7851415,
                  "Y": 0.34377998
                },
                {
                  "X": 0.7678011,
                  "Y": 0.34371206
                }
              ]
            },
            "Id": "be9c9b05-6747-4a7a-9a4f-b7130ad0f13b",
            "Page": 1,
            "Text": "P",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.64027,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014571797,
                "Left": 0.123487644,
                "Top": 0.3464798,
                "Width": 0.03128008
              },
              "Polygon": [
                {
                  "X": 0.12396209,
                  "Y": 0.3464798
                },
                {
                  "X": 0.15476772,
                  "Y": 0.34660143
                },
                {
                  "X": 0.15429299,
                  "Y": 0.36105162
                },
                {
                  "X": 0.123487644,
                  "Y": 0.36092734
                }
              ]
            },
            "Id": "f3e93eed-a921-45e1-a017-2eda28e0f611",
            "Page": 1,
            "Text": "GA",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.15067,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015938628,
                "Left": 0.16308118,
                "Top": 0.34765744,
                "Width": 0.093871765
              },
              "Polygon": [
                {
                  "X": 0.16359249,
                  "Y": 0.34765744
                },
                {
                  "X": 0.25695297,
                  "Y": 0.34802657
                },
                {
                  "X": 0.2564407,
                  "Y": 0.36359608
                },
                {
                  "X": 0.16308118,
                  "Y": 0.36321834
                }
              ]
            },
            "Id": "78ba43bd-5403-4073-90cb-7ba83e0b252c",
            "Page": 1,
            "Text": "LINSEED",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.33785,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01594761,
                "Left": 0.26463124,
                "Top": 0.34930062,
                "Width": 0.07758883
              },
              "Polygon": [
                {
                  "X": 0.26514572,
                  "Y": 0.34930062
                },
                {
                  "X": 0.34222007,
                  "Y": 0.34960592
                },
                {
                  "X": 0.34170476,
                  "Y": 0.36524823
                },
                {
                  "X": 0.26463124,
                  "Y": 0.36493582
                }
              ]
            },
            "Id": "7f52c6e8-f832-4a61-80e7-b3d595e128ec",
            "Page": 1,
            "Text": "REFINE",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 81.64376,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015264325,
                "Left": 0.38062787,
                "Top": 0.3504369,
                "Width": 0.14757542
              },
              "Polygon": [
                {
                  "X": 0.38111138,
                  "Y": 0.3504369
                },
                {
                  "X": 0.5282033,
                  "Y": 0.35102013
                },
                {
                  "X": 0.52771837,
                  "Y": 0.36570123
                },
                {
                  "X": 0.38062787,
                  "Y": 0.3651052
                }
              ]
            },
            "Id": "ebc0dde1-1dc4-40d1-bd18-c61d2cabfffa",
            "Page": 1,
            "Text": "T29911060087",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 78.80073,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015898459,
                "Left": 0.5777256,
                "Top": 0.34852678,
                "Width": 0.0659237
              },
              "Polygon": [
                {
                  "X": 0.5782424,
                  "Y": 0.34852678
                },
                {
                  "X": 0.6436493,
                  "Y": 0.34878507
                },
                {
                  "X": 0.64313185,
                  "Y": 0.36442524
                },
                {
                  "X": 0.5777256,
                  "Y": 0.3641609
                }
              ]
            },
            "Id": "44f2f3cc-552d-47e2-af0f-5a1ae96f0d7b",
            "Page": 1,
            "Text": "15.99",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 35.003166,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013322455,
                "Left": 0.4520505,
                "Top": 0.3652627,
                "Width": 0.04131226
              },
              "Polygon": [
                {
                  "X": 0.45248452,
                  "Y": 0.3652627
                },
                {
                  "X": 0.49336275,
                  "Y": 0.3654283
                },
                {
                  "X": 0.4929284,
                  "Y": 0.37858513
                },
                {
                  "X": 0.4520505,
                  "Y": 0.37841636
                }
              ]
            },
            "Id": "275740ac-87e0-47e8-8678-f98c79616071",
            "Page": 1,
            "Text": "10",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.68304,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01488617,
                "Left": 0.5028342,
                "Top": 0.36401492,
                "Width": 0.06416679
              },
              "Polygon": [
                {
                  "X": 0.50331706,
                  "Y": 0.36401492
                },
                {
                  "X": 0.567001,
                  "Y": 0.36427236
                },
                {
                  "X": 0.5665175,
                  "Y": 0.3789011
                },
                {
                  "X": 0.5028342,
                  "Y": 0.37863812
                }
              ]
            },
            "Id": "97c5cb63-5beb-4d6c-81ed-14ecc322909f",
            "Page": 1,
            "Text": "12.79",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.75601,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015485014,
                "Left": 0.6799429,
                "Top": 0.3610776,
                "Width": 0.066833034
              },
              "Polygon": [
                {
                  "X": 0.6804465,
                  "Y": 0.3610776
                },
                {
                  "X": 0.746776,
                  "Y": 0.36134434
                },
                {
                  "X": 0.74627167,
                  "Y": 0.37656263
                },
                {
                  "X": 0.6799429,
                  "Y": 0.37628993
                }
              ]
            },
            "Id": "89b0004d-f81d-42b9-b4b9-7acf5695e067",
            "Page": 1,
            "Text": "12.79",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.99524,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015332047,
                "Left": 0.22684209,
                "Top": 0.37765265,
                "Width": 0.043123372
              },
              "Polygon": [
                {
                  "X": 0.22734043,
                  "Y": 0.37765265
                },
                {
                  "X": 0.26996547,
                  "Y": 0.37782872
                },
                {
                  "X": 0.2694667,
                  "Y": 0.3929847
                },
                {
                  "X": 0.22684209,
                  "Y": 0.3928048
                }
              ]
            },
            "Id": "bd0914a6-0a6c-4be6-b6d6-42598c9a438c",
            "Page": 1,
            "Text": "CPN",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.69235,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015299104,
                "Left": 0.27731127,
                "Top": 0.37842792,
                "Width": 0.04173878
              },
              "Polygon": [
                {
                  "X": 0.27780905,
                  "Y": 0.37842792
                },
                {
                  "X": 0.31905004,
                  "Y": 0.3785984
                },
                {
                  "X": 0.3185518,
                  "Y": 0.39372703
                },
                {
                  "X": 0.27731127,
                  "Y": 0.39355284
                }
              ]
            },
            "Id": "02740f62-c84a-441d-8838-1bb04b95543c",
            "Page": 1,
            "Text": "GET",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 96.21,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015724123,
                "Left": 0.32968166,
                "Top": 0.37857065,
                "Width": 0.07727808
              },
              "Polygon": [
                {
                  "X": 0.33018887,
                  "Y": 0.37857065
                },
                {
                  "X": 0.40695974,
                  "Y": 0.37888795
                },
                {
                  "X": 0.4064517,
                  "Y": 0.39429477
                },
                {
                  "X": 0.32968166,
                  "Y": 0.39397046
                }
              ]
            },
            "Id": "f476a3ba-689a-49e0-abeb-22beabd449f6",
            "Page": 1,
            "Text": "ITM20%",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.780396,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014642005,
                "Left": 0.4759855,
                "Top": 0.37922654,
                "Width": 0.06355699
              },
              "Polygon": [
                {
                  "X": 0.47645998,
                  "Y": 0.37922654
                },
                {
                  "X": 0.5395425,
                  "Y": 0.3794873
                },
                {
                  "X": 0.5390674,
                  "Y": 0.39386857
                },
                {
                  "X": 0.4759855,
                  "Y": 0.39360243
                }
              ]
            },
            "Id": "b140121c-f6f2-45a5-a1dd-8fa07f795a80",
            "Page": 1,
            "Text": "3.20-",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.844666,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015019411,
                "Left": 0.3772245,
                "Top": 0.39470252,
                "Width": 0.042269755
              },
              "Polygon": [
                {
                  "X": 0.3777136,
                  "Y": 0.39470252
                },
                {
                  "X": 0.41949424,
                  "Y": 0.39487916
                },
                {
                  "X": 0.41900474,
                  "Y": 0.40972194
                },
                {
                  "X": 0.3772245,
                  "Y": 0.4095416
                }
              ]
            },
            "Id": "86fcbec5-54f1-4599-8ab9-47b0bf4a7a7d",
            "Page": 1,
            "Text": "YOU",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 54.6812,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015421658,
                "Left": 0.42775166,
                "Top": 0.39473945,
                "Width": 0.08434843
              },
              "Polygon": [
                {
                  "X": 0.4282484,
                  "Y": 0.39473945
                },
                {
                  "X": 0.5121001,
                  "Y": 0.39509386
                },
                {
                  "X": 0.5116025,
                  "Y": 0.4101611
                },
                {
                  "X": 0.42775166,
                  "Y": 0.4097992
                }
              ]
            },
            "Id": "cafce452-711d-4e0b-8f83-d33de2bf492c",
            "Page": 1,
            "Text": "SAVED $",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.904816,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015397385,
                "Left": 0.54934925,
                "Top": 0.39373937,
                "Width": 0.06848461
              },
              "Polygon": [
                {
                  "X": 0.5498483,
                  "Y": 0.39373937
                },
                {
                  "X": 0.61783385,
                  "Y": 0.39402613
                },
                {
                  "X": 0.61733407,
                  "Y": 0.40913677
                },
                {
                  "X": 0.54934925,
                  "Y": 0.40884393
                }
              ]
            },
            "Id": "43ef554b-5c47-480b-a18b-f42530b4f7ba",
            "Page": 1,
            "Text": "23.19",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.24682,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017108686,
                "Left": 0.121770106,
                "Top": 0.40551037,
                "Width": 0.1196173
              },
              "Polygon": [
                {
                  "X": 0.12231475,
                  "Y": 0.40551037
                },
                {
                  "X": 0.2413874,
                  "Y": 0.4060222
                },
                {
                  "X": 0.24084143,
                  "Y": 0.42261907
                },
                {
                  "X": 0.121770106,
                  "Y": 0.42209554
                }
              ]
            },
            "Id": "9b64043a-4bda-4615-8e44-0db8058854fb",
            "Page": 1,
            "Text": "Coupon(s)",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 75.31794,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016177593,
                "Left": 0.2510383,
                "Top": 0.40744117,
                "Width": 0.102316245
              },
              "Polygon": [
                {
                  "X": 0.25155583,
                  "Y": 0.40744117
                },
                {
                  "X": 0.35335457,
                  "Y": 0.40787956
                },
                {
                  "X": 0.35283598,
                  "Y": 0.42361876
                },
                {
                  "X": 0.2510383,
                  "Y": 0.42317086
                }
              ]
            },
            "Id": "22836ed2-d521-4f7f-8008-eb578698dcb7",
            "Page": 1,
            "Text": "Applied",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 94.45265,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016360579,
                "Left": 0.14754468,
                "Top": 0.42016283,
                "Width": 0.15631245
              },
              "Polygon": [
                {
                  "X": 0.14805923,
                  "Y": 0.42016283
                },
                {
                  "X": 0.30385712,
                  "Y": 0.42084596
                },
                {
                  "X": 0.30334094,
                  "Y": 0.4365234
                },
                {
                  "X": 0.14754468,
                  "Y": 0.4358258
                }
              ]
            },
            "Id": "1222fb41-4c5d-4056-9cef-8f23d9d9608a",
            "Page": 1,
            "Text": "400100949528",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 91.93455,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014241085,
                "Left": 0.32496178,
                "Top": 0.42183962,
                "Width": 0.044000372
              },
              "Polygon": [
                {
                  "X": 0.3254244,
                  "Y": 0.42183962
                },
                {
                  "X": 0.36896217,
                  "Y": 0.42203078
                },
                {
                  "X": 0.36849913,
                  "Y": 0.43608072
                },
                {
                  "X": 0.32496178,
                  "Y": 0.43588597
                }
              ]
            },
            "Id": "d5a9e60d-fac7-4900-bd73-d90db2855ca4",
            "Page": 1,
            "Text": "CPN",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.59936,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015267689,
                "Left": 0.37482643,
                "Top": 0.4207619,
                "Width": 0.040700622
              },
              "Polygon": [
                {
                  "X": 0.3753237,
                  "Y": 0.4207619
                },
                {
                  "X": 0.41552705,
                  "Y": 0.42093807
                },
                {
                  "X": 0.41502935,
                  "Y": 0.43602958
                },
                {
                  "X": 0.37482643,
                  "Y": 0.43584982
                }
              ]
            },
            "Id": "225cca9f-92a6-4917-9a53-d1674446df31",
            "Page": 1,
            "Text": "GET",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 96.90277,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016448008,
                "Left": 0.42732462,
                "Top": 0.420862,
                "Width": 0.07686256
              },
              "Polygon": [
                {
                  "X": 0.42785588,
                  "Y": 0.420862
                },
                {
                  "X": 0.50418717,
                  "Y": 0.42119646
                },
                {
                  "X": 0.5036551,
                  "Y": 0.43731
                },
                {
                  "X": 0.42732462,
                  "Y": 0.43696827
                }
              ]
            },
            "Id": "c4855527-a861-4c1a-b73a-beeefa7275ea",
            "Page": 1,
            "Text": "ITM20%",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.738754,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015416053,
                "Left": 0.28961253,
                "Top": 0.43507823,
                "Width": 0.10022858
              },
              "Polygon": [
                {
                  "X": 0.29010504,
                  "Y": 0.43507823
                },
                {
                  "X": 0.3898411,
                  "Y": 0.435524
                },
                {
                  "X": 0.38934758,
                  "Y": 0.4504943
                },
                {
                  "X": 0.28961253,
                  "Y": 0.45003965
                }
              ]
            },
            "Id": "34766663-8133-4635-8cf2-a35c3a7285de",
            "Page": 1,
            "Text": "SUBTOTAL",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.88723,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0154617885,
                "Left": 0.5111842,
                "Top": 0.4356041,
                "Width": 0.06752795
              },
              "Polygon": [
                {
                  "X": 0.5116848,
                  "Y": 0.4356041
                },
                {
                  "X": 0.57871217,
                  "Y": 0.4359035
                },
                {
                  "X": 0.57821095,
                  "Y": 0.45106587
                },
                {
                  "X": 0.5111842,
                  "Y": 0.45076045
                }
              ]
            },
            "Id": "2fe59bba-772a-4531-878f-616267977f6c",
            "Page": 1,
            "Text": "24.77",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 84.63728,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014630884,
                "Left": 0.21011256,
                "Top": 0.44859707,
                "Width": 0.041100692
              },
              "Polygon": [
                {
                  "X": 0.21058746,
                  "Y": 0.44859707
                },
                {
                  "X": 0.25121325,
                  "Y": 0.448782
                },
                {
                  "X": 0.25073797,
                  "Y": 0.46322796
                },
                {
                  "X": 0.21011256,
                  "Y": 0.46303955
                }
              ]
            },
            "Id": "6381051e-2bac-4bb5-bb63-de7f71596ffd",
            "Page": 1,
            "Text": "PIF",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 96.02063,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013944954,
                "Left": 0.26336095,
                "Top": 0.45025048,
                "Width": 0.051376324
              },
              "Polygon": [
                {
                  "X": 0.26381207,
                  "Y": 0.45025048
                },
                {
                  "X": 0.31473726,
                  "Y": 0.4504827
                },
                {
                  "X": 0.3142857,
                  "Y": 0.46419543
                },
                {
                  "X": 0.26336095,
                  "Y": 0.46395904
                }
              ]
            },
            "Id": "4942dbe1-d371-42eb-9bf5-6bb938a3517e",
            "Page": 1,
            "Text": "1.00",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 79.51501,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0114247,
                "Left": 0.3252197,
                "Top": 0.4514915,
                "Width": 0.014014155
              },
              "Polygon": [
                {
                  "X": 0.3255939,
                  "Y": 0.4514915
                },
                {
                  "X": 0.33923385,
                  "Y": 0.45155376
                },
                {
                  "X": 0.33885953,
                  "Y": 0.4629162
                },
                {
                  "X": 0.3252197,
                  "Y": 0.46285298
                }
              ]
            },
            "Id": "e90ed663-4d3c-41a4-8e43-12d4d63ac969",
            "Page": 1,
            "Text": "%",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 63.02355,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013023372,
                "Left": 0.54264784,
                "Top": 0.45162806,
                "Width": 0.03513037
              },
              "Polygon": [
                {
                  "X": 0.5430728,
                  "Y": 0.45162806
                },
                {
                  "X": 0.5777782,
                  "Y": 0.45178634
                },
                {
                  "X": 0.57735294,
                  "Y": 0.4646514
                },
                {
                  "X": 0.54264784,
                  "Y": 0.46449047
                }
              ]
            },
            "Id": "5e69deea-ba83-495c-b43d-86d75c7d4eac",
            "Page": 1,
            "Text": "25",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.843254,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014603888,
                "Left": 0.20980147,
                "Top": 0.4635684,
                "Width": 0.10248282
              },
              "Polygon": [
                {
                  "X": 0.21026582,
                  "Y": 0.4635684
                },
                {
                  "X": 0.3122843,
                  "Y": 0.46404183
                },
                {
                  "X": 0.31181896,
                  "Y": 0.47817227
                },
                {
                  "X": 0.20980147,
                  "Y": 0.47769028
                }
              ]
            },
            "Id": "2bb7ca27-94fd-4d36-af86-61da98142b5e",
            "Page": 1,
            "Text": "SUBTOTAL",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 85.27774,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013516507,
                "Left": 0.3229733,
                "Top": 0.463369,
                "Width": 0.064072266
              },
              "Polygon": [
                {
                  "X": 0.3234086,
                  "Y": 0.463369
                },
                {
                  "X": 0.38704556,
                  "Y": 0.46366405
                },
                {
                  "X": 0.3866097,
                  "Y": 0.4768855
                },
                {
                  "X": 0.3229733,
                  "Y": 0.47658548
                }
              ]
            },
            "Id": "1d81d94a-1915-44f5-862e-8f6ede7f6091",
            "Page": 1,
            "Text": "W/PIF",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.997604,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013931713,
                "Left": 0.51215535,
                "Top": 0.46471348,
                "Width": 0.06702088
              },
              "Polygon": [
                {
                  "X": 0.5126051,
                  "Y": 0.46471348
                },
                {
                  "X": 0.57917625,
                  "Y": 0.46502233
                },
                {
                  "X": 0.5787259,
                  "Y": 0.4786452
                },
                {
                  "X": 0.51215535,
                  "Y": 0.478331
                }
              ]
            },
            "Id": "f0cdc3e3-c5c7-493f-940e-8fa96e274148",
            "Page": 1,
            "Text": "25.02",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.78356,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01557251,
                "Left": 0.20817952,
                "Top": 0.47752836,
                "Width": 0.06792904
              },
              "Polygon": [
                {
                  "X": 0.20868088,
                  "Y": 0.47752836
                },
                {
                  "X": 0.27610856,
                  "Y": 0.4778469
                },
                {
                  "X": 0.27560648,
                  "Y": 0.49310088
                },
                {
                  "X": 0.20817952,
                  "Y": 0.49277627
                }
              ]
            },
            "Id": "63da16f3-eb74-4b0a-b394-1605b29b3cd7",
            "Page": 1,
            "Text": "Sales",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.75603,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014227885,
                "Left": 0.28524798,
                "Top": 0.47839558,
                "Width": 0.04096382
              },
              "Polygon": [
                {
                  "X": 0.28570992,
                  "Y": 0.47839558
                },
                {
                  "X": 0.32621178,
                  "Y": 0.478587
                },
                {
                  "X": 0.32574946,
                  "Y": 0.49262345
                },
                {
                  "X": 0.28524798,
                  "Y": 0.49242866
                }
              ]
            },
            "Id": "50c7724f-9036-418b-84f1-4575b85a778e",
            "Page": 1,
            "Text": "Tax",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 86.15666,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014683647,
                "Left": 0.33537576,
                "Top": 0.4780407,
                "Width": 0.054817308
              },
              "Polygon": [
                {
                  "X": 0.33585086,
                  "Y": 0.4780407
                },
                {
                  "X": 0.39019307,
                  "Y": 0.47829735
                },
                {
                  "X": 0.38971743,
                  "Y": 0.49272433
                },
                {
                  "X": 0.33537576,
                  "Y": 0.49246302
                }
              ]
            },
            "Id": "21aba29b-fba1-4440-917e-52733f389e88",
            "Page": 1,
            "Text": "6.5%",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.7047,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013707429,
                "Left": 0.524055,
                "Top": 0.47901157,
                "Width": 0.05524973
              },
              "Polygon": [
                {
                  "X": 0.52449906,
                  "Y": 0.47901157
                },
                {
                  "X": 0.5793047,
                  "Y": 0.47927043
                },
                {
                  "X": 0.57886016,
                  "Y": 0.492719
                },
                {
                  "X": 0.524055,
                  "Y": 0.49245575
                }
              ]
            },
            "Id": "96dc5f08-210e-4bbc-a3c4-2b9c512f636b",
            "Page": 1,
            "Text": "1.63",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.81102,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015007724,
                "Left": 0.32206234,
                "Top": 0.49233186,
                "Width": 0.06629092
              },
              "Polygon": [
                {
                  "X": 0.32254604,
                  "Y": 0.49233186
                },
                {
                  "X": 0.38835326,
                  "Y": 0.49264827
                },
                {
                  "X": 0.38786894,
                  "Y": 0.5073396
                },
                {
                  "X": 0.32206234,
                  "Y": 0.50701743
                }
              ]
            },
            "Id": "5bc3c123-6580-463d-bc75-e7c4c779a860",
            "Page": 1,
            "Text": "TOTAL",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 82.55806,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0149916215,
                "Left": 0.5115432,
                "Top": 0.49297485,
                "Width": 0.06800821
              },
              "Polygon": [
                {
                  "X": 0.5120274,
                  "Y": 0.49297485
                },
                {
                  "X": 0.5795514,
                  "Y": 0.4932994
                },
                {
                  "X": 0.5790666,
                  "Y": 0.50796646
                },
                {
                  "X": 0.5115432,
                  "Y": 0.507636
                }
              ]
            },
            "Id": "b15d5fc1-9f6e-4d93-b1e2-fc69dd900c18",
            "Page": 1,
            "Text": "26.65",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.80154,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015616526,
                "Left": 0.11348839,
                "Top": 0.5106172,
                "Width": 0.09370454
              },
              "Polygon": [
                {
                  "X": 0.113985844,
                  "Y": 0.5106172
                },
                {
                  "X": 0.20719293,
                  "Y": 0.51107603
                },
                {
                  "X": 0.20669453,
                  "Y": 0.52623373
                },
                {
                  "X": 0.11348839,
                  "Y": 0.5257665
                }
              ]
            },
            "Id": "000c3d92-ca49-4e96-b2e3-d7a3c1e94489",
            "Page": 1,
            "Text": "ACCOUNT",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.4596,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016261311,
                "Left": 0.21651095,
                "Top": 0.5108411,
                "Width": 0.082637474
              },
              "Polygon": [
                {
                  "X": 0.21703218,
                  "Y": 0.5108411
                },
                {
                  "X": 0.29914844,
                  "Y": 0.5112452
                },
                {
                  "X": 0.29862633,
                  "Y": 0.5271024
                },
                {
                  "X": 0.21651095,
                  "Y": 0.52669054
                }
              ]
            },
            "Id": "dd753112-ac06-45eb-89eb-0366c2a5befb",
            "Page": 1,
            "Text": "NUMBER",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.37107,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014611467,
                "Left": 0.1518607,
                "Top": 0.52709156,
                "Width": 0.057538606
              },
              "Polygon": [
                {
                  "X": 0.15233119,
                  "Y": 0.52709156
                },
                {
                  "X": 0.2093993,
                  "Y": 0.527378
                },
                {
                  "X": 0.20892826,
                  "Y": 0.541703
                },
                {
                  "X": 0.1518607,
                  "Y": 0.5414117
                }
              ]
            },
            "Id": "dd83761b-b012-4b51-b289-80c4bc2da201",
            "Page": 1,
            "Text": "Visa",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.98092,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015559327,
                "Left": 0.5106136,
                "Top": 0.5266225,
                "Width": 0.06945804
              },
              "Polygon": [
                {
                  "X": 0.51111585,
                  "Y": 0.5266225
                },
                {
                  "X": 0.5800716,
                  "Y": 0.52696764
                },
                {
                  "X": 0.5795687,
                  "Y": 0.5421818
                },
                {
                  "X": 0.5106136,
                  "Y": 0.54183036
                }
              ]
            },
            "Id": "f2a7a2e9-684f-445d-9662-8e86e4d5fee1",
            "Page": 1,
            "Text": "26.65",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.26795,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015328728,
                "Left": 0.1491293,
                "Top": 0.54136807,
                "Width": 0.11779974
              },
              "Polygon": [
                {
                  "X": 0.1496129,
                  "Y": 0.54136807
                },
                {
                  "X": 0.26692903,
                  "Y": 0.5419669
                },
                {
                  "X": 0.26644427,
                  "Y": 0.5566968
                },
                {
                  "X": 0.1491293,
                  "Y": 0.5560877
                }
              ]
            },
            "Id": "edafaf31-b69b-43aa-b454-fd73b6d79bfa",
            "Page": 1,
            "Text": "APPROVAL",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 94.81589,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015652599,
                "Left": 0.2797406,
                "Top": 0.5403332,
                "Width": 0.08159661
              },
              "Polygon": [
                {
                  "X": 0.280242,
                  "Y": 0.5403332
                },
                {
                  "X": 0.3613372,
                  "Y": 0.5407464
                },
                {
                  "X": 0.360835,
                  "Y": 0.5559858
                },
                {
                  "X": 0.2797406,
                  "Y": 0.55556536
                }
              ]
            },
            "Id": "96913de3-e024-47d9-9dc0-00e9db1f99ec",
            "Page": 1,
            "Text": "00751C",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.03094,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015535188,
                "Left": 0.3683331,
                "Top": 0.5399316,
                "Width": 0.055761665
              },
              "Polygon": [
                {
                  "X": 0.3688357,
                  "Y": 0.5399316
                },
                {
                  "X": 0.42409477,
                  "Y": 0.5402128
                },
                {
                  "X": 0.4235916,
                  "Y": 0.5554668
                },
                {
                  "X": 0.3683331,
                  "Y": 0.55518055
                }
              ]
            },
            "Id": "f46e8a21-6eb3-44c8-a3f0-19ebcb78c12b",
            "Page": 1,
            "Text": "CHIP",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.54395,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015713394,
                "Left": 0.43334576,
                "Top": 0.54027927,
                "Width": 0.079687804
              },
              "Polygon": [
                {
                  "X": 0.4338506,
                  "Y": 0.54027927
                },
                {
                  "X": 0.51303357,
                  "Y": 0.54068226
                },
                {
                  "X": 0.5125279,
                  "Y": 0.55599266
                },
                {
                  "X": 0.43334576,
                  "Y": 0.55558246
                }
              ]
            },
            "Id": "3eaa6ecc-980a-40a1-8928-780c879b904e",
            "Page": 1,
            "Text": "ONLINE",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.17218,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015206193,
                "Left": 0.16235839,
                "Top": 0.5562802,
                "Width": 0.14684653
              },
              "Polygon": [
                {
                  "X": 0.16283269,
                  "Y": 0.5562802
                },
                {
                  "X": 0.30920494,
                  "Y": 0.5570403
                },
                {
                  "X": 0.3087292,
                  "Y": 0.5714864
                },
                {
                  "X": 0.16235839,
                  "Y": 0.5707138
                }
              ]
            },
            "Id": "8d3acceb-dd6b-4850-838f-d13ae2b95c62",
            "Page": 1,
            "Text": "Application",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 90.845314,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0154200895,
                "Left": 0.31816807,
                "Top": 0.5562052,
                "Width": 0.07444805
              },
              "Polygon": [
                {
                  "X": 0.3186631,
                  "Y": 0.5562052
                },
                {
                  "X": 0.39261612,
                  "Y": 0.5565888
                },
                {
                  "X": 0.39212036,
                  "Y": 0.5716253
                },
                {
                  "X": 0.31816807,
                  "Y": 0.57123506
                }
              ]
            },
            "Id": "e9a3f929-a77f-4646-abd7-2e6d2c61d3d1",
            "Page": 1,
            "Text": "Label:",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.801476,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015063472,
                "Left": 0.4073417,
                "Top": 0.55582255,
                "Width": 0.05625859
              },
              "Polygon": [
                {
                  "X": 0.40782875,
                  "Y": 0.55582255
                },
                {
                  "X": 0.46360028,
                  "Y": 0.5561116
                },
                {
                  "X": 0.46311268,
                  "Y": 0.570886
                },
                {
                  "X": 0.4073417,
                  "Y": 0.5705921
                }
              ]
            },
            "Id": "0ad4221e-da4c-4cb9-8479-b3468a8f1078",
            "Page": 1,
            "Text": "VISA",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.8386,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015673574,
                "Left": 0.47161973,
                "Top": 0.55581355,
                "Width": 0.079096586
              },
              "Polygon": [
                {
                  "X": 0.47212338,
                  "Y": 0.55581355
                },
                {
                  "X": 0.5507163,
                  "Y": 0.5562207
                },
                {
                  "X": 0.5502118,
                  "Y": 0.5714871
                },
                {
                  "X": 0.47161973,
                  "Y": 0.5710729
                }
              ]
            },
            "Id": "408c2d79-c8a4-42c3-a0f9-af8f81715593",
            "Page": 1,
            "Text": "CREDIT",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 90.91459,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014453372,
                "Left": 0.16144538,
                "Top": 0.5713028,
                "Width": 0.051828112
              },
              "Polygon": [
                {
                  "X": 0.16191126,
                  "Y": 0.5713028
                },
                {
                  "X": 0.2132735,
                  "Y": 0.5715741
                },
                {
                  "X": 0.2128071,
                  "Y": 0.5857562
                },
                {
                  "X": 0.16144538,
                  "Y": 0.5854806
                }
              ]
            },
            "Id": "824aeb91-e330-4b9f-990e-56946ff592d3",
            "Page": 1,
            "Text": "AID:",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.0503,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015811654,
                "Left": 0.22805136,
                "Top": 0.5704451,
                "Width": 0.18154532
              },
              "Polygon": [
                {
                  "X": 0.2285395,
                  "Y": 0.5704451
                },
                {
                  "X": 0.40959668,
                  "Y": 0.57140017
                },
                {
                  "X": 0.40910673,
                  "Y": 0.5862568
                },
                {
                  "X": 0.22805136,
                  "Y": 0.58528584
                }
              ]
            },
            "Id": "6f8772c4-e365-4bb4-957e-b347d03f3aec",
            "Page": 1,
            "Text": "A0000000031010",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 81.71538,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014592211,
                "Left": 0.16102745,
                "Top": 0.58616924,
                "Width": 0.05113281
              },
              "Polygon": [
                {
                  "X": 0.16149788,
                  "Y": 0.58616924
                },
                {
                  "X": 0.21216026,
                  "Y": 0.58644134
                },
                {
                  "X": 0.21168934,
                  "Y": 0.6007615
                },
                {
                  "X": 0.16102745,
                  "Y": 0.6004851
                }
              ]
            },
            "Id": "a1b1c0bf-ef74-4480-addb-02d406e95f7d",
            "Page": 1,
            "Text": "TVR:",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.51331,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015093621,
                "Left": 0.22527339,
                "Top": 0.5866089,
                "Width": 0.13348089
              },
              "Polygon": [
                {
                  "X": 0.22574596,
                  "Y": 0.5866089
                },
                {
                  "X": 0.35875428,
                  "Y": 0.58732325
                },
                {
                  "X": 0.35828042,
                  "Y": 0.6017025
                },
                {
                  "X": 0.22527339,
                  "Y": 0.6009768
                }
              ]
            },
            "Id": "721053f8-d26d-4c79-beef-3ab14a9c6baa",
            "Page": 1,
            "Text": "0880008000",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 72.34961,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015161799,
                "Left": 0.1609792,
                "Top": 0.6024514,
                "Width": 0.04871006
              },
              "Polygon": [
                {
                  "X": 0.16146863,
                  "Y": 0.6024514
                },
                {
                  "X": 0.20968926,
                  "Y": 0.602715
                },
                {
                  "X": 0.20919934,
                  "Y": 0.6176132
                },
                {
                  "X": 0.1609792,
                  "Y": 0.61734533
                }
              ]
            },
            "Id": "85bd580c-e48d-4ba4-9e4d-1ecf1c6c839b",
            "Page": 1,
            "Text": "TSI:",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.86517,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014472546,
                "Left": 0.22399686,
                "Top": 0.6031706,
                "Width": 0.05672073
              },
              "Polygon": [
                {
                  "X": 0.2244626,
                  "Y": 0.6031706
                },
                {
                  "X": 0.28071758,
                  "Y": 0.60347825
                },
                {
                  "X": 0.28025132,
                  "Y": 0.6176431
                },
                {
                  "X": 0.22399686,
                  "Y": 0.61733073
                }
              ]
            },
            "Id": "8a8ca841-70e5-43ad-8f4e-ad79998505e1",
            "Page": 1,
            "Text": "E800",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.90476,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016086454,
                "Left": 0.16511393,
                "Top": 0.62220615,
                "Width": 0.055159498
              },
              "Polygon": [
                {
                  "X": 0.16563238,
                  "Y": 0.62220615
                },
                {
                  "X": 0.22027342,
                  "Y": 0.62251127
                },
                {
                  "X": 0.21975438,
                  "Y": 0.6382926
                },
                {
                  "X": 0.16511393,
                  "Y": 0.63798237
                }
              ]
            },
            "Id": "bc30a0f9-b3f3-412a-80a5-570f3f075242",
            "Page": 1,
            "Text": "This",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.96995,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016777687,
                "Left": 0.2276804,
                "Top": 0.6223306,
                "Width": 0.09407569
              },
              "Polygon": [
                {
                  "X": 0.2282148,
                  "Y": 0.6223306
                },
                {
                  "X": 0.3217561,
                  "Y": 0.6228528
                },
                {
                  "X": 0.3212207,
                  "Y": 0.6391083
                },
                {
                  "X": 0.2276804,
                  "Y": 0.63857704
                }
              ]
            },
            "Id": "63f8325e-13f8-4435-b23e-51ad86f73922",
            "Page": 1,
            "Text": "receipt",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.77642,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016060662,
                "Left": 0.33183256,
                "Top": 0.6229215,
                "Width": 0.09589933
              },
              "Polygon": [
                {
                  "X": 0.3323438,
                  "Y": 0.6229215
                },
                {
                  "X": 0.42773187,
                  "Y": 0.62345403
                },
                {
                  "X": 0.42721963,
                  "Y": 0.6389822
                },
                {
                  "X": 0.33183256,
                  "Y": 0.63844085
                }
              ]
            },
            "Id": "634936aa-ae04-4a64-aed8-42f9beaeccc5",
            "Page": 1,
            "Text": "expires",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.86791,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015304389,
                "Left": 0.43750346,
                "Top": 0.62268555,
                "Width": 0.028032754
              },
              "Polygon": [
                {
                  "X": 0.43800324,
                  "Y": 0.62268555
                },
                {
                  "X": 0.4655362,
                  "Y": 0.6228391
                },
                {
                  "X": 0.46503615,
                  "Y": 0.63798994
                },
                {
                  "X": 0.43750346,
                  "Y": 0.6378339
                }
              ]
            },
            "Id": "1cb11244-6020-45cb-9ec0-e8cbf4d2bbe0",
            "Page": 1,
            "Text": "at",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.232285,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015650231,
                "Left": 0.4766733,
                "Top": 0.6219444,
                "Width": 0.030209346
              },
              "Polygon": [
                {
                  "X": 0.47718439,
                  "Y": 0.6219444
                },
                {
                  "X": 0.50688267,
                  "Y": 0.62210995
                },
                {
                  "X": 0.50637126,
                  "Y": 0.6375947
                },
                {
                  "X": 0.4766733,
                  "Y": 0.6374265
                }
              ]
            },
            "Id": "c888542e-d478-4547-ac00-5b1346090266",
            "Page": 1,
            "Text": "60",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 34.44739,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014659723,
                "Left": 0.5156676,
                "Top": 0.62246597,
                "Width": 0.05532068
              },
              "Polygon": [
                {
                  "X": 0.51614153,
                  "Y": 0.62246597
                },
                {
                  "X": 0.5709883,
                  "Y": 0.6227717
                },
                {
                  "X": 0.5705138,
                  "Y": 0.63712573
                },
                {
                  "X": 0.5156676,
                  "Y": 0.6368153
                }
              ]
            },
            "Id": "d48cd37b-82ca-41e6-ae37-19fcede8ea8a",
            "Page": 1,
            "Text": "days",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.57415,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.011527227,
                "Left": 0.57981503,
                "Top": 0.62545323,
                "Width": 0.028769443
              },
              "Polygon": [
                {
                  "X": 0.5801908,
                  "Y": 0.62545323
                },
                {
                  "X": 0.60858446,
                  "Y": 0.62561196
                },
                {
                  "X": 0.6082085,
                  "Y": 0.6369805
                },
                {
                  "X": 0.57981503,
                  "Y": 0.63681984
                }
              ]
            },
            "Id": "ba5e8762-c7b3-421e-847c-1467fba0e29a",
            "Page": 1,
            "Text": "on",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.8502,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017056476,
                "Left": 0.6162085,
                "Top": 0.6203366,
                "Width": 0.10812018
              },
              "Polygon": [
                {
                  "X": 0.61675256,
                  "Y": 0.6203366
                },
                {
                  "X": 0.72432864,
                  "Y": 0.62093455
                },
                {
                  "X": 0.72378343,
                  "Y": 0.63739306
                },
                {
                  "X": 0.6162085,
                  "Y": 0.6367847
                }
              ]
            },
            "Id": "1930000e-4651-4837-b7a3-ebe7d43fb6ae",
            "Page": 1,
            "Text": "03/04/22",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.434074,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015702253,
                "Left": 0.18844669,
                "Top": 0.6389885,
                "Width": 0.10678916
              },
              "Polygon": [
                {
                  "X": 0.18894272,
                  "Y": 0.6389885
                },
                {
                  "X": 0.29523584,
                  "Y": 0.6395926
                },
                {
                  "X": 0.29473874,
                  "Y": 0.6546908
                },
                {
                  "X": 0.18844669,
                  "Y": 0.65407723
                }
              ]
            },
            "Id": "0ccb3e42-0e9a-4d24-9a99-9a16c62dfed1",
            "Page": 1,
            "Text": "Previous",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.55867,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015854405,
                "Left": 0.30444047,
                "Top": 0.6382706,
                "Width": 0.109751984
              },
              "Polygon": [
                {
                  "X": 0.3049418,
                  "Y": 0.6382706
                },
                {
                  "X": 0.41419247,
                  "Y": 0.63889056
                },
                {
                  "X": 0.41369,
                  "Y": 0.654125
                },
                {
                  "X": 0.30444047,
                  "Y": 0.6534952
                }
              ]
            },
            "Id": "e615db66-a78e-4f2f-bef3-f670cf8dca90",
            "Page": 1,
            "Text": "Michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.10089,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015370329,
                "Left": 0.42377576,
                "Top": 0.6376282,
                "Width": 0.095233314
              },
              "Polygon": [
                {
                  "X": 0.4242648,
                  "Y": 0.6376282
                },
                {
                  "X": 0.51900905,
                  "Y": 0.6381651
                },
                {
                  "X": 0.5185191,
                  "Y": 0.6529985
                },
                {
                  "X": 0.42377576,
                  "Y": 0.6524533
                }
              ]
            },
            "Id": "7d184f33-e3ec-41fe-ba2a-6909b844ef18",
            "Page": 1,
            "Text": "Rewards",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 91.0034,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015638608,
                "Left": 0.5286872,
                "Top": 0.6372425,
                "Width": 0.101909995
              },
              "Polygon": [
                {
                  "X": 0.5291846,
                  "Y": 0.6372425
                },
                {
                  "X": 0.6305972,
                  "Y": 0.6378166
                },
                {
                  "X": 0.6300988,
                  "Y": 0.6528811
                },
                {
                  "X": 0.5286872,
                  "Y": 0.652298
                }
              ]
            },
            "Id": "b03955f8-7596-45d4-b7b2-acd7191776eb",
            "Page": 1,
            "Text": "Balance:",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.59415,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014781379,
                "Left": 0.64129055,
                "Top": 0.63695055,
                "Width": 0.055129234
              },
              "Polygon": [
                {
                  "X": 0.6417693,
                  "Y": 0.63695055
                },
                {
                  "X": 0.6964198,
                  "Y": 0.63725966
                },
                {
                  "X": 0.6959405,
                  "Y": 0.65173197
                },
                {
                  "X": 0.64129055,
                  "Y": 0.6514182
                }
              ]
            },
            "Id": "6fc32356-2209-4a95-be2a-b42b916a5133",
            "Page": 1,
            "Text": "$0.0",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.82705,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016338898,
                "Left": 0.15292211,
                "Top": 0.65695894,
                "Width": 0.07374186
              },
              "Polygon": [
                {
                  "X": 0.15344481,
                  "Y": 0.65695894
                },
                {
                  "X": 0.22666398,
                  "Y": 0.6573829
                },
                {
                  "X": 0.22614048,
                  "Y": 0.6732978
                },
                {
                  "X": 0.15292211,
                  "Y": 0.6728669
                }
              ]
            },
            "Id": "1b169f8a-08fb-4fb1-820e-af7fa8f7c041",
            "Page": 1,
            "Text": "Click",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 85.147385,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0155722825,
                "Left": 0.24481598,
                "Top": 0.6576197,
                "Width": 0.0466176
              },
              "Polygon": [
                {
                  "X": 0.2453194,
                  "Y": 0.6576197
                },
                {
                  "X": 0.29143357,
                  "Y": 0.6578868
                },
                {
                  "X": 0.29092968,
                  "Y": 0.673192
                },
                {
                  "X": 0.24481598,
                  "Y": 0.67292076
                }
              ]
            },
            "Id": "f4974966-2878-42bc-9761-93a2467db24a",
            "Page": 1,
            "Text": "Buy",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.03405,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014831924,
                "Left": 0.31132904,
                "Top": 0.6563837,
                "Width": 0.085462414
              },
              "Polygon": [
                {
                  "X": 0.31180108,
                  "Y": 0.6563837
                },
                {
                  "X": 0.39679146,
                  "Y": 0.6568751
                },
                {
                  "X": 0.3963186,
                  "Y": 0.6712156
                },
                {
                  "X": 0.31132904,
                  "Y": 0.67071694
                }
              ]
            },
            "Id": "30086c7d-b386-44a7-829d-b1a4d49c6259",
            "Page": 1,
            "Text": "Create",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 78.76596,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015226759,
                "Left": 0.41691214,
                "Top": 0.6549359,
                "Width": 0.055582337
              },
              "Polygon": [
                {
                  "X": 0.4174037,
                  "Y": 0.6549359
                },
                {
                  "X": 0.47249445,
                  "Y": 0.6552537
                },
                {
                  "X": 0.47200233,
                  "Y": 0.6701626
                },
                {
                  "X": 0.41691214,
                  "Y": 0.6698399
                }
              ]
            },
            "Id": "d5490f7e-840c-4f31-9bfb-c6640e9b7dbf",
            "Page": 1,
            "Text": "Shop",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 93.78323,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014950724,
                "Left": 0.4807843,
                "Top": 0.6559888,
                "Width": 0.10953854
              },
              "Polygon": [
                {
                  "X": 0.48125678,
                  "Y": 0.6559888
                },
                {
                  "X": 0.59032285,
                  "Y": 0.65661854
                },
                {
                  "X": 0.5898493,
                  "Y": 0.6709395
                },
                {
                  "X": 0.4807843,
                  "Y": 0.67030054
                }
              ]
            },
            "Id": "9ad2901e-2a19-4b02-bef0-4f5df63f2421",
            "Page": 1,
            "Text": "michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 57.14264,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.011670256,
                "Left": 0.59743536,
                "Top": 0.6580025,
                "Width": 0.041646406
              },
              "Polygon": [
                {
                  "X": 0.5978133,
                  "Y": 0.6580025
                },
                {
                  "X": 0.6390817,
                  "Y": 0.6582411
                },
                {
                  "X": 0.63870347,
                  "Y": 0.6696727
                },
                {
                  "X": 0.59743536,
                  "Y": 0.6694313
                }
              ]
            },
            "Id": "cf1ae385-00d9-42cd-82ea-01ff61a63f62",
            "Page": 1,
            "Text": "com",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 83.22647,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016972367,
                "Left": 0.6494241,
                "Top": 0.65510887,
                "Width": 0.074561715
              },
              "Polygon": [
                {
                  "X": 0.64997137,
                  "Y": 0.65510887
                },
                {
                  "X": 0.7239858,
                  "Y": 0.6555354
                },
                {
                  "X": 0.7234376,
                  "Y": 0.67208123
                },
                {
                  "X": 0.6494241,
                  "Y": 0.6716474
                }
              ]
            },
            "Id": "05527001-d1e5-47d8-b78e-b7cd4301e3ac",
            "Page": 1,
            "Text": "today!",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.830536,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015109536,
                "Left": 0.1178321,
                "Top": 0.6765591,
                "Width": 0.041576397
              },
              "Polygon": [
                {
                  "X": 0.118320234,
                  "Y": 0.6765591
                },
                {
                  "X": 0.1594085,
                  "Y": 0.67680186
                },
                {
                  "X": 0.15891996,
                  "Y": 0.6916686
                },
                {
                  "X": 0.1178321,
                  "Y": 0.6914222
                }
              ]
            },
            "Id": "325d3c70-df0a-460e-ba59-795214f5ecd2",
            "Page": 1,
            "Text": "Get",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.672035,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015385802,
                "Left": 0.17151783,
                "Top": 0.67593306,
                "Width": 0.09713195
              },
              "Polygon": [
                {
                  "X": 0.17200448,
                  "Y": 0.67593306
                },
                {
                  "X": 0.2686498,
                  "Y": 0.67650354
                },
                {
                  "X": 0.26816216,
                  "Y": 0.69131887
                },
                {
                  "X": 0.17151783,
                  "Y": 0.69073987
                }
              ]
            },
            "Id": "854fa964-5351-4418-99a8-a2de84a9a3b4",
            "Page": 1,
            "Text": "Savings",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 92.03821,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.012823093,
                "Left": 0.27882823,
                "Top": 0.6748198,
                "Width": 0.017106688
              },
              "Polygon": [
                {
                  "X": 0.27924705,
                  "Y": 0.6748198
                },
                {
                  "X": 0.29593492,
                  "Y": 0.6749182
                },
                {
                  "X": 0.29551592,
                  "Y": 0.68764293
                },
                {
                  "X": 0.27882823,
                  "Y": 0.68754333
                }
              ]
            },
            "Id": "01016d9a-7a68-469e-adb9-b09d93193fca",
            "Page": 1,
            "Text": "&",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 72.22706,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015305119,
                "Left": 0.3069325,
                "Top": 0.67276275,
                "Width": 0.15547581
              },
              "Polygon": [
                {
                  "X": 0.30740607,
                  "Y": 0.67276275
                },
                {
                  "X": 0.4624083,
                  "Y": 0.6736741
                },
                {
                  "X": 0.46193326,
                  "Y": 0.6880679
                },
                {
                  "X": 0.3069325,
                  "Y": 0.6871434
                }
              ]
            },
            "Id": "9965d26c-13ca-4bde-b7b2-ea83e675bab7",
            "Page": 1,
            "Text": "Inspiration",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 96.22499,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014468839,
                "Left": 0.47571796,
                "Top": 0.67343754,
                "Width": 0.066882245
              },
              "Polygon": [
                {
                  "X": 0.47618252,
                  "Y": 0.67343754
                },
                {
                  "X": 0.5426002,
                  "Y": 0.6738279
                },
                {
                  "X": 0.542135,
                  "Y": 0.6879064
                },
                {
                  "X": 0.47571796,
                  "Y": 0.6875105
                }
              ]
            },
            "Id": "4cf15bd5-1884-4c29-a887-1d9ceec7d45f",
            "Page": 1,
            "Text": "Text*",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 89.3965,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014661548,
                "Left": 0.5527745,
                "Top": 0.6725847,
                "Width": 0.07829946
              },
              "Polygon": [
                {
                  "X": 0.55324376,
                  "Y": 0.6725847
                },
                {
                  "X": 0.63107395,
                  "Y": 0.6730416
                },
                {
                  "X": 0.630604,
                  "Y": 0.68724626
                },
                {
                  "X": 0.5527745,
                  "Y": 0.6867829
                }
              ]
            },
            "Id": "498a6065-5051-4c82-a2a6-5b9fc14245c3",
            "Page": 1,
            "Text": "SIGNUP",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.45805,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013901407,
                "Left": 0.64289045,
                "Top": 0.67339057,
                "Width": 0.026921578
              },
              "Polygon": [
                {
                  "X": 0.64334524,
                  "Y": 0.67339057
                },
                {
                  "X": 0.669812,
                  "Y": 0.67354596
                },
                {
                  "X": 0.66935694,
                  "Y": 0.687292
                },
                {
                  "X": 0.64289045,
                  "Y": 0.6871344
                }
              ]
            },
            "Id": "4a790e7c-e4ab-4137-b784-1353123004ad",
            "Page": 1,
            "Text": "to",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.7588,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017292835,
                "Left": 0.67955685,
                "Top": 0.67303723,
                "Width": 0.08215712
              },
              "Polygon": [
                {
                  "X": 0.68011326,
                  "Y": 0.67303723
                },
                {
                  "X": 0.761714,
                  "Y": 0.67351604
                },
                {
                  "X": 0.7611566,
                  "Y": 0.6903301
                },
                {
                  "X": 0.67955685,
                  "Y": 0.6898431
                }
              ]
            },
            "Id": "a848e46b-568c-4a47-b4ae-e451cae28829",
            "Page": 1,
            "Text": "273283",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.56549,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014153984,
                "Left": 0.20067248,
                "Top": 0.68961203,
                "Width": 0.028376305
              },
              "Polygon": [
                {
                  "X": 0.20113231,
                  "Y": 0.68961203
                },
                {
                  "X": 0.22904879,
                  "Y": 0.68977904
                },
                {
                  "X": 0.2285887,
                  "Y": 0.703766
                },
                {
                  "X": 0.20067248,
                  "Y": 0.70359665
                }
              ]
            },
            "Id": "62ba6f0e-e64c-4bf6-b7da-f21234523ace",
            "Page": 1,
            "Text": "To",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.58349,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015688023,
                "Left": 0.23892625,
                "Top": 0.6885813,
                "Width": 0.056836575
              },
              "Polygon": [
                {
                  "X": 0.23943113,
                  "Y": 0.6885813
                },
                {
                  "X": 0.29576284,
                  "Y": 0.68891793
                },
                {
                  "X": 0.29525736,
                  "Y": 0.70426935
                },
                {
                  "X": 0.23892625,
                  "Y": 0.7039276
                }
              ]
            },
            "Id": "e80b1281-efc8-4736-a57b-ad242bc6e75b",
            "Page": 1,
            "Text": "Sign",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 93.44113,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013964448,
                "Left": 0.30578613,
                "Top": 0.688262,
                "Width": 0.02950729
              },
              "Polygon": [
                {
                  "X": 0.30624023,
                  "Y": 0.688262
                },
                {
                  "X": 0.33529344,
                  "Y": 0.6884355
                },
                {
                  "X": 0.3348391,
                  "Y": 0.70222646
                },
                {
                  "X": 0.30578613,
                  "Y": 0.7020506
                }
              ]
            },
            "Id": "f489dd8a-7938-4909-ac4b-0d44fe4619f5",
            "Page": 1,
            "Text": "Up",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.10235,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014683425,
                "Left": 0.34485078,
                "Top": 0.68726295,
                "Width": 0.042292245
              },
              "Polygon": [
                {
                  "X": 0.34532624,
                  "Y": 0.68726295
                },
                {
                  "X": 0.38714302,
                  "Y": 0.68751234
                },
                {
                  "X": 0.38666713,
                  "Y": 0.7019464
                },
                {
                  "X": 0.34485078,
                  "Y": 0.70169336
                }
              ]
            },
            "Id": "0f35c8b1-07c1-4f22-b204-cf09ba9d3e69",
            "Page": 1,
            "Text": "for",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.70228,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015639076,
                "Left": 0.39669326,
                "Top": 0.6871428,
                "Width": 0.0659171
              },
              "Polygon": [
                {
                  "X": 0.3971959,
                  "Y": 0.6871428
                },
                {
                  "X": 0.46261036,
                  "Y": 0.6875327
                },
                {
                  "X": 0.46210706,
                  "Y": 0.70278186
                },
                {
                  "X": 0.39669326,
                  "Y": 0.70238596
                }
              ]
            },
            "Id": "a4b5df56-c08c-44a3-b94e-71a3b444d96f",
            "Page": 1,
            "Text": "Email",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 58.747017,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014217106,
                "Left": 0.47402668,
                "Top": 0.6882922,
                "Width": 0.016788516
              },
              "Polygon": [
                {
                  "X": 0.47449276,
                  "Y": 0.6882922
                },
                {
                  "X": 0.4908152,
                  "Y": 0.6883896
                },
                {
                  "X": 0.490349,
                  "Y": 0.70250934
                },
                {
                  "X": 0.47402668,
                  "Y": 0.7024106
                }
              ]
            },
            "Id": "2a432d42-6ccc-4abd-86c6-8b2f22e44a19",
            "Page": 1,
            "Text": "&",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.837776,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015613213,
                "Left": 0.49943197,
                "Top": 0.68804526,
                "Width": 0.053813975
              },
              "Polygon": [
                {
                  "X": 0.4999369,
                  "Y": 0.68804526
                },
                {
                  "X": 0.55324596,
                  "Y": 0.68836313
                },
                {
                  "X": 0.55274045,
                  "Y": 0.70365846
                },
                {
                  "X": 0.49943197,
                  "Y": 0.70333576
                }
              ]
            },
            "Id": "83065128-fe04-4ad2-9a29-a585b7cb6eee",
            "Page": 1,
            "Text": "Text",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 78.9277,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014934324,
                "Left": 0.5614677,
                "Top": 0.6882543,
                "Width": 0.115297705
              },
              "Polygon": [
                {
                  "X": 0.56193835,
                  "Y": 0.6882543
                },
                {
                  "X": 0.6767654,
                  "Y": 0.688939
                },
                {
                  "X": 0.6762936,
                  "Y": 0.70318866
                },
                {
                  "X": 0.5614677,
                  "Y": 0.7024943
                }
              ]
            },
            "Id": "88ee480e-96d6-4407-bc26-fe28fed0742f",
            "Page": 1,
            "Text": "Messages",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 70.48144,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015073376,
                "Left": 0.2665825,
                "Top": 0.70273453,
                "Width": 0.054876845
              },
              "Polygon": [
                {
                  "X": 0.2670676,
                  "Y": 0.70273453
                },
                {
                  "X": 0.32145935,
                  "Y": 0.7030641
                },
                {
                  "X": 0.3209737,
                  "Y": 0.71780795
                },
                {
                  "X": 0.2665825,
                  "Y": 0.7174736
                }
              ]
            },
            "Id": "61401ac1-caf1-44c5-8162-81035d7dd11b",
            "Page": 1,
            "Text": "*Msg",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.18177,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013000212,
                "Left": 0.33163407,
                "Top": 0.7026362,
                "Width": 0.015885243
              },
              "Polygon": [
                {
                  "X": 0.33205923,
                  "Y": 0.7026362
                },
                {
                  "X": 0.34751934,
                  "Y": 0.7027298
                },
                {
                  "X": 0.34709406,
                  "Y": 0.71563643
                },
                {
                  "X": 0.33163407,
                  "Y": 0.7155416
                }
              ]
            },
            "Id": "1bca35ca-1079-4a01-ae99-05fe81445769",
            "Page": 1,
            "Text": "&",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 69.43619,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0144905755,
                "Left": 0.35810882,
                "Top": 0.702322,
                "Width": 0.054609004
              },
              "Polygon": [
                {
                  "X": 0.3585754,
                  "Y": 0.702322
                },
                {
                  "X": 0.41271782,
                  "Y": 0.7026497
                },
                {
                  "X": 0.4122507,
                  "Y": 0.7168126
                },
                {
                  "X": 0.35810882,
                  "Y": 0.7164803
                }
              ]
            },
            "Id": "0aa5d288-3fc8-45bb-bb99-6f60c99ba1d0",
            "Page": 1,
            "Text": "Date",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.7568,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015213119,
                "Left": 0.422302,
                "Top": 0.7026674,
                "Width": 0.06755286
              },
              "Polygon": [
                {
                  "X": 0.42279026,
                  "Y": 0.7026674
                },
                {
                  "X": 0.48985487,
                  "Y": 0.7030733
                },
                {
                  "X": 0.48936597,
                  "Y": 0.7178805
                },
                {
                  "X": 0.422302,
                  "Y": 0.71746874
                }
              ]
            },
            "Id": "0dcb191f-dd26-4bd9-8c1e-8aa5f8832eea",
            "Page": 1,
            "Text": "Rates",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 50.398834,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013388408,
                "Left": 0.49945647,
                "Top": 0.70498043,
                "Width": 0.041935906
              },
              "Polygon": [
                {
                  "X": 0.49989018,
                  "Y": 0.70498043
                },
                {
                  "X": 0.5413924,
                  "Y": 0.7052321
                },
                {
                  "X": 0.5409583,
                  "Y": 0.7183689
                },
                {
                  "X": 0.49945647,
                  "Y": 0.718114
                }
              ]
            },
            "Id": "a0f96620-d531-48e6-b9ce-b58db04828ea",
            "Page": 1,
            "Text": "May",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.62271,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0145798605,
                "Left": 0.5491149,
                "Top": 0.70292884,
                "Width": 0.06721129
              },
              "Polygon": [
                {
                  "X": 0.5495832,
                  "Y": 0.70292884
                },
                {
                  "X": 0.61632615,
                  "Y": 0.7033326
                },
                {
                  "X": 0.61585724,
                  "Y": 0.71750873
                },
                {
                  "X": 0.5491149,
                  "Y": 0.71709937
                }
              ]
            },
            "Id": "c46a1bf9-b3ed-4d7e-bb57-e0138d937d90",
            "Page": 1,
            "Text": "Apply",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.03973,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013547504,
                "Left": 0.19896232,
                "Top": 0.71704334,
                "Width": 0.0447096
              },
              "Polygon": [
                {
                  "X": 0.19939871,
                  "Y": 0.71704334
                },
                {
                  "X": 0.24367192,
                  "Y": 0.71731544
                },
                {
                  "X": 0.24323513,
                  "Y": 0.7305909
                },
                {
                  "X": 0.19896232,
                  "Y": 0.73031527
                }
              ]
            },
            "Id": "80b7bb38-f809-46fc-aa29-0d874a49d181",
            "Page": 1,
            "Text": "You",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.339355,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013231039,
                "Left": 0.25187916,
                "Top": 0.7174627,
                "Width": 0.053503867
              },
              "Polygon": [
                {
                  "X": 0.25230366,
                  "Y": 0.7174627
                },
                {
                  "X": 0.30538303,
                  "Y": 0.71778893
                },
                {
                  "X": 0.30495808,
                  "Y": 0.73069376
                },
                {
                  "X": 0.25187916,
                  "Y": 0.7303634
                }
              ]
            },
            "Id": "2b8f0af8-95a3-4a34-beb9-5be4772ffc00",
            "Page": 1,
            "Text": "will",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.74081,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014141948,
                "Left": 0.31897157,
                "Top": 0.7173307,
                "Width": 0.093260415
              },
              "Polygon": [
                {
                  "X": 0.31941834,
                  "Y": 0.7173307
                },
                {
                  "X": 0.41223198,
                  "Y": 0.71790093
                },
                {
                  "X": 0.41178438,
                  "Y": 0.7314727
                },
                {
                  "X": 0.31897157,
                  "Y": 0.73089504
                }
              ]
            },
            "Id": "0c3ffd65-ab3e-4846-97fa-bfa59d4d34f5",
            "Page": 1,
            "Text": "receive",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 87.24036,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013490522,
                "Left": 0.42420223,
                "Top": 0.7180641,
                "Width": 0.011378643
              },
              "Polygon": [
                {
                  "X": 0.424645,
                  "Y": 0.7180641
                },
                {
                  "X": 0.43558088,
                  "Y": 0.71813136
                },
                {
                  "X": 0.43513802,
                  "Y": 0.7315547
                },
                {
                  "X": 0.42420223,
                  "Y": 0.7314866
                }
              ]
            },
            "Id": "171138bf-a44d-4449-a84d-3777a2b2a1f7",
            "Page": 1,
            "Text": "1",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 94.70462,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015413037,
                "Left": 0.4483868,
                "Top": 0.7176807,
                "Width": 0.12956402
              },
              "Polygon": [
                {
                  "X": 0.44886887,
                  "Y": 0.7176807
                },
                {
                  "X": 0.5779508,
                  "Y": 0.7184734
                },
                {
                  "X": 0.57746744,
                  "Y": 0.73309374
                },
                {
                  "X": 0.4483868,
                  "Y": 0.7322899
                }
              ]
            },
            "Id": "0fc12bed-1ca0-4827-9546-8ab7a10d4c32",
            "Page": 1,
            "Text": "autodialed",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 88.19353,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014378735,
                "Left": 0.58584785,
                "Top": 0.7192826,
                "Width": 0.09394817
              },
              "Polygon": [
                {
                  "X": 0.58630407,
                  "Y": 0.7192826
                },
                {
                  "X": 0.67979604,
                  "Y": 0.71985716
                },
                {
                  "X": 0.679339,
                  "Y": 0.73366135
                },
                {
                  "X": 0.58584785,
                  "Y": 0.7330792
                }
              ]
            },
            "Id": "e3e96cc6-e86c-465e-8eff-0da20c8abd00",
            "Page": 1,
            "Text": "message",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.26333,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013416481,
                "Left": 0.20430528,
                "Top": 0.73077005,
                "Width": 0.05875778
              },
              "Polygon": [
                {
                  "X": 0.20473436,
                  "Y": 0.73077005
                },
                {
                  "X": 0.26306304,
                  "Y": 0.7311333
                },
                {
                  "X": 0.26263344,
                  "Y": 0.7441865
                },
                {
                  "X": 0.20430528,
                  "Y": 0.74381876
                }
              ]
            },
            "Id": "1d9ce091-86fe-4033-afac-3659b9d01374",
            "Page": 1,
            "Text": "with",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.93804,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.010348531,
                "Left": 0.2723656,
                "Top": 0.73361826,
                "Width": 0.015744172
              },
              "Polygon": [
                {
                  "X": 0.27270305,
                  "Y": 0.73361826
                },
                {
                  "X": 0.28810978,
                  "Y": 0.7337144
                },
                {
                  "X": 0.28777224,
                  "Y": 0.74396676
                },
                {
                  "X": 0.2723656,
                  "Y": 0.74386966
                }
              ]
            },
            "Id": "3185d70c-17ae-44d9-a281-406942a52e80",
            "Page": 1,
            "Text": "a",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 91.09801,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015008711,
                "Left": 0.3001473,
                "Top": 0.73075,
                "Width": 0.053890288
              },
              "Polygon": [
                {
                  "X": 0.30063042,
                  "Y": 0.73075
                },
                {
                  "X": 0.35403758,
                  "Y": 0.7310824
                },
                {
                  "X": 0.35355392,
                  "Y": 0.7457587
                },
                {
                  "X": 0.3001473,
                  "Y": 0.74542165
                }
              ]
            },
            "Id": "c9bc2dcb-368d-4fb7-b0d4-d4d38299f3f5",
            "Page": 1,
            "Text": "link",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.14336,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013980776,
                "Left": 0.36445916,
                "Top": 0.7319289,
                "Width": 0.027472034
              },
              "Polygon": [
                {
                  "X": 0.36491433,
                  "Y": 0.7319289
                },
                {
                  "X": 0.3919312,
                  "Y": 0.73209715
                },
                {
                  "X": 0.39147577,
                  "Y": 0.7459097
                },
                {
                  "X": 0.36445916,
                  "Y": 0.74573916
                }
              ]
            },
            "Id": "b356121b-3b06-407b-839f-a3d36310be11",
            "Page": 1,
            "Text": "to",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 62.776173,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01531775,
                "Left": 0.40325502,
                "Top": 0.73252904,
                "Width": 0.053639606
              },
              "Polygon": [
                {
                  "X": 0.40374908,
                  "Y": 0.73252904
                },
                {
                  "X": 0.4568946,
                  "Y": 0.73286015
                },
                {
                  "X": 0.4564,
                  "Y": 0.7478468
                },
                {
                  "X": 0.40325502,
                  "Y": 0.74751097
                }
              ]
            },
            "Id": "355e2a34-36ee-43b6-929b-5c79116bc414",
            "Page": 1,
            "Text": "join",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 96.66618,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015653905,
                "Left": 0.4655408,
                "Top": 0.731763,
                "Width": 0.103529856
              },
              "Polygon": [
                {
                  "X": 0.46603602,
                  "Y": 0.731763
                },
                {
                  "X": 0.56907064,
                  "Y": 0.73240423
                },
                {
                  "X": 0.56857437,
                  "Y": 0.7474169
                },
                {
                  "X": 0.4655408,
                  "Y": 0.7467665
                }
              ]
            },
            "Id": "236c7312-69a4-452b-8ec7-26b859001ec6",
            "Page": 1,
            "Text": "Michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 81.72601,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015614984,
                "Left": 0.58015364,
                "Top": 0.7315533,
                "Width": 0.08468781
              },
              "Polygon": [
                {
                  "X": 0.58065236,
                  "Y": 0.7315533
                },
                {
                  "X": 0.6648415,
                  "Y": 0.7320768
                },
                {
                  "X": 0.66434187,
                  "Y": 0.7471683
                },
                {
                  "X": 0.58015364,
                  "Y": 0.7466373
                }
              ]
            },
            "Id": "e03fb7d4-6af4-41b8-8c87-d339ca346b44",
            "Page": 1,
            "Text": "alerts",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 72.29077,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016106134,
                "Left": 0.3475108,
                "Top": 0.74939,
                "Width": 0.06981609
              },
              "Polygon": [
                {
                  "X": 0.34802687,
                  "Y": 0.74939
                },
                {
                  "X": 0.41732693,
                  "Y": 0.7498289
                },
                {
                  "X": 0.41681013,
                  "Y": 0.76549613
                },
                {
                  "X": 0.3475108,
                  "Y": 0.7650509
                }
              ]
            },
            "Id": "0f89a2d0-f542-4873-954f-9aabb5ed640c",
            "Page": 1,
            "Text": "Baron",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.49856,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0162427,
                "Left": 0.42516044,
                "Top": 0.75052303,
                "Width": 0.10549448
              },
              "Polygon": [
                {
                  "X": 0.425674,
                  "Y": 0.75052303
                },
                {
                  "X": 0.5306549,
                  "Y": 0.7511882
                },
                {
                  "X": 0.5301402,
                  "Y": 0.7667657
                },
                {
                  "X": 0.42516044,
                  "Y": 0.76609087
                }
              ]
            },
            "Id": "3bba5cae-18ad-4a26-a665-4bac3ebbc85f",
            "Page": 1,
            "Text": "Brothers",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.44343,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015288427,
                "Left": 0.34758976,
                "Top": 0.7648174,
                "Width": 0.08033254
              },
              "Polygon": [
                {
                  "X": 0.3480764,
                  "Y": 0.7648174
                },
                {
                  "X": 0.4279223,
                  "Y": 0.7653304
                },
                {
                  "X": 0.42743486,
                  "Y": 0.7801059
                },
                {
                  "X": 0.34758976,
                  "Y": 0.77958596
                }
              ]
            },
            "Id": "f3a6862d-799a-4d13-89de-5bdeffd6ee61",
            "Page": 1,
            "Text": "Custom",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 86.27665,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015041584,
                "Left": 0.43640208,
                "Top": 0.7656449,
                "Width": 0.09461723
              },
              "Polygon": [
                {
                  "X": 0.43687814,
                  "Y": 0.7656449
                },
                {
                  "X": 0.53101933,
                  "Y": 0.7662498
                },
                {
                  "X": 0.5305424,
                  "Y": 0.7806865
                },
                {
                  "X": 0.43640208,
                  "Y": 0.7800735
                }
              ]
            },
            "Id": "99f143d6-092e-4111-bb81-66927e99c498",
            "Page": 1,
            "Text": "Franing",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 83.00299,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014258644,
                "Left": 0.13510326,
                "Top": 0.7748327,
                "Width": 0.053266153
              },
              "Polygon": [
                {
                  "X": 0.13556026,
                  "Y": 0.7748327
                },
                {
                  "X": 0.18836941,
                  "Y": 0.7751755
                },
                {
                  "X": 0.1879119,
                  "Y": 0.78909135
                },
                {
                  "X": 0.13510326,
                  "Y": 0.78874415
                }
              ]
            },
            "Id": "fbd3ad71-b19c-4969-8bcd-cde3f563859a",
            "Page": 1,
            "Text": "New!",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.667496,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015288142,
                "Left": 0.20066121,
                "Top": 0.775517,
                "Width": 0.041957222
              },
              "Polygon": [
                {
                  "X": 0.20115495,
                  "Y": 0.775517
                },
                {
                  "X": 0.24261844,
                  "Y": 0.7757862
                },
                {
                  "X": 0.24212429,
                  "Y": 0.7908051
                },
                {
                  "X": 0.20066121,
                  "Y": 0.7905322
                }
              ]
            },
            "Id": "e6b14515-4b7f-4929-ae9c-ce7077a4461c",
            "Page": 1,
            "Text": "Now",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.43791,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013938792,
                "Left": 0.2546859,
                "Top": 0.7780035,
                "Width": 0.025586389
              },
              "Polygon": [
                {
                  "X": 0.25513917,
                  "Y": 0.7780035
                },
                {
                  "X": 0.2802723,
                  "Y": 0.77816707
                },
                {
                  "X": 0.27981883,
                  "Y": 0.79194236
                },
                {
                  "X": 0.2546859,
                  "Y": 0.7917768
                }
              ]
            },
            "Id": "9f1ba886-bf2f-404e-803c-6d74075edce8",
            "Page": 1,
            "Text": "in",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.22392,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01649631,
                "Left": 0.2895056,
                "Top": 0.77731884,
                "Width": 0.053526416
              },
              "Polygon": [
                {
                  "X": 0.29003724,
                  "Y": 0.77731884
                },
                {
                  "X": 0.34303203,
                  "Y": 0.7776633
                },
                {
                  "X": 0.34249982,
                  "Y": 0.79381514
                },
                {
                  "X": 0.2895056,
                  "Y": 0.79346555
                }
              ]
            },
            "Id": "5aefff8b-f19b-4425-9bae-7ddff54918cd",
            "Page": 1,
            "Text": "Over",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.70869,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01629288,
                "Left": 0.3541946,
                "Top": 0.77982044,
                "Width": 0.068133675
              },
              "Polygon": [
                {
                  "X": 0.35471684,
                  "Y": 0.77982044
                },
                {
                  "X": 0.4223283,
                  "Y": 0.78026074
                },
                {
                  "X": 0.42180535,
                  "Y": 0.7961133
                },
                {
                  "X": 0.3541946,
                  "Y": 0.79566664
                }
              ]
            },
            "Id": "f71ad56e-ed34-4e0a-a5d9-ebb7b3c53128",
            "Page": 1,
            "Text": "1,200",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.629036,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015352938,
                "Left": 0.43053368,
                "Top": 0.77970135,
                "Width": 0.10618941
              },
              "Polygon": [
                {
                  "X": 0.4310172,
                  "Y": 0.77970135
                },
                {
                  "X": 0.5367231,
                  "Y": 0.7803894
                },
                {
                  "X": 0.53623855,
                  "Y": 0.7950543
                },
                {
                  "X": 0.43053368,
                  "Y": 0.79435706
                }
              ]
            },
            "Id": "3f639ebb-97d4-4622-8147-2ce1f04333ba",
            "Page": 1,
            "Text": "Michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.14067,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015854517,
                "Left": 0.5451137,
                "Top": 0.77861685,
                "Width": 0.08006858
              },
              "Polygon": [
                {
                  "X": 0.54562026,
                  "Y": 0.77861685
                },
                {
                  "X": 0.6251823,
                  "Y": 0.77913386
                },
                {
                  "X": 0.6246748,
                  "Y": 0.7944714
                },
                {
                  "X": 0.5451137,
                  "Y": 0.79394716
                }
              ]
            },
            "Id": "2868f44c-99fe-4a85-ba73-ab6daf3fb063",
            "Page": 1,
            "Text": "Stores",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.24292,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015332124,
                "Left": 0.6354073,
                "Top": 0.77949685,
                "Width": 0.016491348
              },
              "Polygon": [
                {
                  "X": 0.63591117,
                  "Y": 0.77949685
                },
                {
                  "X": 0.6518987,
                  "Y": 0.7796008
                },
                {
                  "X": 0.65139467,
                  "Y": 0.794829
                },
                {
                  "X": 0.6354073,
                  "Y": 0.79472363
                }
              ]
            },
            "Id": "ce4c895c-fe07-410a-a603-3ab67b5715b8",
            "Page": 1,
            "Text": "&",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.84601,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017688064,
                "Left": 0.66071635,
                "Top": 0.77974814,
                "Width": 0.08140254
              },
              "Polygon": [
                {
                  "X": 0.6612842,
                  "Y": 0.77974814
                },
                {
                  "X": 0.74211895,
                  "Y": 0.78027356
                },
                {
                  "X": 0.74155015,
                  "Y": 0.7974362
                },
                {
                  "X": 0.66071635,
                  "Y": 0.79690254
                }
              ]
            },
            "Id": "47733e78-27a6-4141-a93d-1f6cb5be4c3b",
            "Page": 1,
            "Text": "Online",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 84.4502,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015980002,
                "Left": 0.19058007,
                "Top": 0.80481243,
                "Width": 0.043241713
              },
              "Polygon": [
                {
                  "X": 0.19109596,
                  "Y": 0.80481243
                },
                {
                  "X": 0.23382178,
                  "Y": 0.8050973
                },
                {
                  "X": 0.23330544,
                  "Y": 0.82079244
                },
                {
                  "X": 0.19058007,
                  "Y": 0.8205036
                }
              ]
            },
            "Id": "25256a21-fe0a-4496-ba3a-3b70099d19af",
            "Page": 1,
            "Text": "Now",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 84.52916,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.019323425,
                "Left": 0.24261115,
                "Top": 0.80583215,
                "Width": 0.08971778
              },
              "Polygon": [
                {
                  "X": 0.24322706,
                  "Y": 0.80583215
                },
                {
                  "X": 0.33232895,
                  "Y": 0.8064266
                },
                {
                  "X": 0.33171192,
                  "Y": 0.8251556
                },
                {
                  "X": 0.24261115,
                  "Y": 0.8245513
                }
              ]
            },
            "Id": "06b4f131-be81-4aa5-ae02-2139556ca5af",
            "Page": 1,
            "Text": "Hiring!",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.61851,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017930835,
                "Left": 0.34578016,
                "Top": 0.808736,
                "Width": 0.07021145
              },
              "Polygon": [
                {
                  "X": 0.34635544,
                  "Y": 0.808736
                },
                {
                  "X": 0.4159916,
                  "Y": 0.80920154
                },
                {
                  "X": 0.4154155,
                  "Y": 0.8266669
                },
                {
                  "X": 0.34578016,
                  "Y": 0.82619417
                }
              ]
            },
            "Id": "72823059-fd2c-451d-8c74-87e2a1782f9f",
            "Page": 1,
            "Text": "Apply",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.19166,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015544738,
                "Left": 0.42353383,
                "Top": 0.80919814,
                "Width": 0.026721297
              },
              "Polygon": [
                {
                  "X": 0.42404076,
                  "Y": 0.80919814
                },
                {
                  "X": 0.45025513,
                  "Y": 0.8093734
                },
                {
                  "X": 0.4497479,
                  "Y": 0.8247429
                },
                {
                  "X": 0.42353383,
                  "Y": 0.8245653
                }
              ]
            },
            "Id": "1553246b-0e00-4e94-b602-e738fde2f21f",
            "Page": 1,
            "Text": "at",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.141624,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.020130185,
                "Left": 0.46005422,
                "Top": 0.8064932,
                "Width": 0.22183654
              },
              "Polygon": [
                {
                  "X": 0.46066916,
                  "Y": 0.8064932
                },
                {
                  "X": 0.6818908,
                  "Y": 0.8079681
                },
                {
                  "X": 0.68127304,
                  "Y": 0.8266234
                },
                {
                  "X": 0.46005422,
                  "Y": 0.82512414
                }
              ]
            },
            "Id": "b786d103-fac9-4d0c-9bbb-5bfc1b3f4aa6",
            "Page": 1,
            "Text": "michaels.com/jobs",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.50404,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015345482,
                "Left": 0.21753618,
                "Top": 0.8357368,
                "Width": 0.065072745
              },
              "Polygon": [
                {
                  "X": 0.21802618,
                  "Y": 0.8357368
                },
                {
                  "X": 0.28260893,
                  "Y": 0.8361792
                },
                {
                  "X": 0.2821183,
                  "Y": 0.85108227
                },
                {
                  "X": 0.21753618,
                  "Y": 0.85063416
                }
              ]
            },
            "Id": "5f1e4906-04ba-477f-b8ef-2d3cc7311d11",
            "Page": 1,
            "Text": "THANK",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.3143,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015681405,
                "Left": 0.29211262,
                "Top": 0.8374859,
                "Width": 0.04324007
              },
              "Polygon": [
                {
                  "X": 0.29261917,
                  "Y": 0.8374859
                },
                {
                  "X": 0.3353527,
                  "Y": 0.8377789
                },
                {
                  "X": 0.3348457,
                  "Y": 0.8531673
                },
                {
                  "X": 0.29211262,
                  "Y": 0.85287035
                }
              ]
            },
            "Id": "548b4674-9022-40c5-880c-db76bb3e93ea",
            "Page": 1,
            "Text": "YOU",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.89938,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015079023,
                "Left": 0.34443614,
                "Top": 0.83786887,
                "Width": 0.042438116
              },
              "Polygon": [
                {
                  "X": 0.3449234,
                  "Y": 0.83786887
                },
                {
                  "X": 0.38687426,
                  "Y": 0.8381565
                },
                {
                  "X": 0.38638657,
                  "Y": 0.8529479
                },
                {
                  "X": 0.34443614,
                  "Y": 0.85265654
                }
              ]
            },
            "Id": "36927511-715a-43da-96f7-ea6507f25823",
            "Page": 1,
            "Text": "FOR",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.84306,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016886992,
                "Left": 0.39676094,
                "Top": 0.8360695,
                "Width": 0.10537719
              },
              "Polygon": [
                {
                  "X": 0.39729384,
                  "Y": 0.8360695
                },
                {
                  "X": 0.50213814,
                  "Y": 0.83678716
                },
                {
                  "X": 0.50160414,
                  "Y": 0.85295653
                },
                {
                  "X": 0.39676094,
                  "Y": 0.8522288
                }
              ]
            },
            "Id": "5ede9d08-60ca-436c-a756-3958342afcf7",
            "Page": 1,
            "Text": "SHOPPING",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.850555,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015023406,
                "Left": 0.5107481,
                "Top": 0.8361196,
                "Width": 0.028487254
              },
              "Polygon": [
                {
                  "X": 0.5112379,
                  "Y": 0.8361196
                },
                {
                  "X": 0.53923535,
                  "Y": 0.8363111
                },
                {
                  "X": 0.5387452,
                  "Y": 0.851143
                },
                {
                  "X": 0.5107481,
                  "Y": 0.85094905
                }
              ]
            },
            "Id": "a2c25ddb-1910-4c7c-9d28-90b9c8c69625",
            "Page": 1,
            "Text": "AT",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.567825,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017833758,
                "Left": 0.54893327,
                "Top": 0.8367028,
                "Width": 0.107259266
              },
              "Polygon": [
                {
                  "X": 0.5494982,
                  "Y": 0.8367028
                },
                {
                  "X": 0.65619254,
                  "Y": 0.83743286
                },
                {
                  "X": 0.6556264,
                  "Y": 0.8545366
                },
                {
                  "X": 0.54893327,
                  "Y": 0.85379577
                }
              ]
            },
            "Id": "5408fb6f-6e03-403f-bc65-7c48f5d58922",
            "Page": 1,
            "Text": "MICHAELS",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.779366,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015308587,
                "Left": 0.298673,
                "Top": 0.8558416,
                "Width": 0.055062234
              },
              "Polygon": [
                {
                  "X": 0.29916444,
                  "Y": 0.8558416
                },
                {
                  "X": 0.35373524,
                  "Y": 0.85622174
                },
                {
                  "X": 0.35324326,
                  "Y": 0.87115014
                },
                {
                  "X": 0.298673,
                  "Y": 0.8707652
                }
              ]
            },
            "Id": "7199eaff-6aa5-453d-a8f5-f0451d2c25b7",
            "Page": 1,
            "Text": "Dear",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.038605,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015554737,
                "Left": 0.36267945,
                "Top": 0.85550636,
                "Width": 0.08152897
              },
              "Polygon": [
                {
                  "X": 0.36317334,
                  "Y": 0.85550636
                },
                {
                  "X": 0.4442084,
                  "Y": 0.85607046
                },
                {
                  "X": 0.44371375,
                  "Y": 0.8710611
                },
                {
                  "X": 0.36267945,
                  "Y": 0.8704898
                }
              ]
            },
            "Id": "345930c1-2ebb-44e8-ba63-515eff98496a",
            "Page": 1,
            "Text": "Valued",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 77.73774,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016322691,
                "Left": 0.45322052,
                "Top": 0.8550879,
                "Width": 0.114643365
              },
              "Polygon": [
                {
                  "X": 0.45373267,
                  "Y": 0.8550879
                },
                {
                  "X": 0.5678639,
                  "Y": 0.85588163
                },
                {
                  "X": 0.5673505,
                  "Y": 0.87141055
                },
                {
                  "X": 0.45322052,
                  "Y": 0.87060624
                }
              ]
            },
            "Id": "18a31233-6afe-4258-8e6c-79cdb71e70db",
            "Page": 1,
            "Text": "Customer:",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.04203,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016298909,
                "Left": 0.12025268,
                "Top": 0.86588544,
                "Width": 0.10558258
              },
              "Polygon": [
                {
                  "X": 0.120763436,
                  "Y": 0.86588544
                },
                {
                  "X": 0.22583526,
                  "Y": 0.8666244
                },
                {
                  "X": 0.22532341,
                  "Y": 0.8821843
                },
                {
                  "X": 0.12025268,
                  "Y": 0.88143563
                }
              ]
            },
            "Id": "4bc11598-89fb-4f9a-b283-4bbdf4e8b4aa",
            "Page": 1,
            "Text": "Michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.9129,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014851693,
                "Left": 0.23491502,
                "Top": 0.8704866,
                "Width": 0.08107406
              },
              "Polygon": [
                {
                  "X": 0.23538472,
                  "Y": 0.8704866
                },
                {
                  "X": 0.31598908,
                  "Y": 0.87105536
                },
                {
                  "X": 0.31551862,
                  "Y": 0.8853383
                },
                {
                  "X": 0.23491502,
                  "Y": 0.8847628
                }
              ]
            },
            "Id": "edca59bf-f528-4cfd-8a46-458ed696960c",
            "Page": 1,
            "Text": "return",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.932846,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01409672,
                "Left": 0.32485396,
                "Top": 0.8708775,
                "Width": 0.04299058
              },
              "Polygon": [
                {
                  "X": 0.32530835,
                  "Y": 0.8708775
                },
                {
                  "X": 0.36784455,
                  "Y": 0.87117755
                },
                {
                  "X": 0.36738977,
                  "Y": 0.88497424
                },
                {
                  "X": 0.32485396,
                  "Y": 0.8846707
                }
              ]
            },
            "Id": "787fb150-84fc-45d4-a51b-3252cfb6a1a9",
            "Page": 1,
            "Text": "and",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.989914,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01322789,
                "Left": 0.37586054,
                "Top": 0.8726797,
                "Width": 0.08108365
              },
              "Polygon": [
                {
                  "X": 0.37627766,
                  "Y": 0.8726797
                },
                {
                  "X": 0.4569442,
                  "Y": 0.8732494
                },
                {
                  "X": 0.4565264,
                  "Y": 0.8859076
                },
                {
                  "X": 0.37586054,
                  "Y": 0.8853318
                }
              ]
            },
            "Id": "6ac8bc51-c291-4c21-ae27-ca4e0e47fd04",
            "Page": 1,
            "Text": "coupon",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.3869,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015802044,
                "Left": 0.46455035,
                "Top": 0.8704572,
                "Width": 0.10588603
              },
              "Polygon": [
                {
                  "X": 0.46504715,
                  "Y": 0.8704572
                },
                {
                  "X": 0.57043636,
                  "Y": 0.8711997
                },
                {
                  "X": 0.5699385,
                  "Y": 0.88625926
                },
                {
                  "X": 0.46455035,
                  "Y": 0.8855073
                }
              ]
            },
            "Id": "26333a73-5e6e-4261-9c6e-5f0103b73a1a",
            "Page": 1,
            "Text": "policies",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 50.16504,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.011853445,
                "Left": 0.58060175,
                "Top": 0.87512463,
                "Width": 0.039540716
              },
              "Polygon": [
                {
                  "X": 0.5809844,
                  "Y": 0.87512463
                },
                {
                  "X": 0.62014246,
                  "Y": 0.87540144
                },
                {
                  "X": 0.61975944,
                  "Y": 0.8869781
                },
                {
                  "X": 0.58060175,
                  "Y": 0.8866986
                }
              ]
            },
            "Id": "aff7a959-319d-4552-9168-3d9b618196a4",
            "Page": 1,
            "Text": "and",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.6248,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017258957,
                "Left": 0.63386,
                "Top": 0.873482,
                "Width": 0.11761826
              },
              "Polygon": [
                {
                  "X": 0.6344034,
                  "Y": 0.873482
                },
                {
                  "X": 0.75147825,
                  "Y": 0.8743081
                },
                {
                  "X": 0.7509336,
                  "Y": 0.89074093
                },
                {
                  "X": 0.63386,
                  "Y": 0.8899034
                }
              ]
            },
            "Id": "a28179c2-0a2e-4bef-949c-ac49cf84a229",
            "Page": 1,
            "Text": "available",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 95.95692,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.012048066,
                "Left": 0.16466166,
                "Top": 0.88339496,
                "Width": 0.025883269
              },
              "Polygon": [
                {
                  "X": 0.1650516,
                  "Y": 0.88339496
                },
                {
                  "X": 0.19054492,
                  "Y": 0.88357687
                },
                {
                  "X": 0.19015478,
                  "Y": 0.895443
                },
                {
                  "X": 0.16466166,
                  "Y": 0.8952593
                }
              ]
            },
            "Id": "4b32a67e-ff94-4509-a5cf-e5f13f3124f0",
            "Page": 1,
            "Text": "at",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 65.18938,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015331897,
                "Left": 0.20161605,
                "Top": 0.8849914,
                "Width": 0.11082116
              },
              "Polygon": [
                {
                  "X": 0.202094,
                  "Y": 0.8849914
                },
                {
                  "X": 0.3124372,
                  "Y": 0.88577956
                },
                {
                  "X": 0.3119582,
                  "Y": 0.9003233
                },
                {
                  "X": 0.20161605,
                  "Y": 0.8995256
                }
              ]
            },
            "Id": "80b672e1-3d46-45ed-a31c-b7c136227a51",
            "Page": 1,
            "Text": "Michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 52.376823,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.012237223,
                "Left": 0.3174926,
                "Top": 0.8870199,
                "Width": 0.043780286
              },
              "Polygon": [
                {
                  "X": 0.31788537,
                  "Y": 0.8870199
                },
                {
                  "X": 0.3612729,
                  "Y": 0.8873301
                },
                {
                  "X": 0.36087978,
                  "Y": 0.8992571
                },
                {
                  "X": 0.3174926,
                  "Y": 0.89894384
                }
              ]
            },
            "Id": "17f16524-1576-4797-96e1-4424e69c15df",
            "Page": 1,
            "Text": "COM",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.82681,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0134573905,
                "Left": 0.369095,
                "Top": 0.88504666,
                "Width": 0.042668033
              },
              "Polygon": [
                {
                  "X": 0.36952856,
                  "Y": 0.88504666
                },
                {
                  "X": 0.41176304,
                  "Y": 0.885348
                },
                {
                  "X": 0.4113291,
                  "Y": 0.898504
                },
                {
                  "X": 0.369095,
                  "Y": 0.8981993
                }
              ]
            },
            "Id": "318e2217-e62e-4edb-a491-fa2275d29795",
            "Page": 1,
            "Text": "and",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.21674,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.012370369,
                "Left": 0.42228934,
                "Top": 0.8863213,
                "Width": 0.028073037
              },
              "Polygon": [
                {
                  "X": 0.42269087,
                  "Y": 0.8863213
                },
                {
                  "X": 0.45036238,
                  "Y": 0.88651896
                },
                {
                  "X": 0.44996065,
                  "Y": 0.8986917
                },
                {
                  "X": 0.42228934,
                  "Y": 0.8984921
                }
              ]
            },
            "Id": "ccd140f1-b893-4042-87f7-d4036fb01086",
            "Page": 1,
            "Text": "in",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.78419,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013872701,
                "Left": 0.45885298,
                "Top": 0.8852841,
                "Width": 0.06627462
              },
              "Polygon": [
                {
                  "X": 0.45929518,
                  "Y": 0.8852841
                },
                {
                  "X": 0.5251276,
                  "Y": 0.88575375
                },
                {
                  "X": 0.5246848,
                  "Y": 0.8991568
                },
                {
                  "X": 0.45885298,
                  "Y": 0.89868194
                }
              ]
            },
            "Id": "48a67235-16d3-408d-8db7-8a017ee66b11",
            "Page": 1,
            "Text": "store",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.518845,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01347847,
                "Left": 0.53482085,
                "Top": 0.88688844,
                "Width": 0.027192768
              },
              "Polygon": [
                {
                  "X": 0.53525984,
                  "Y": 0.88688844
                },
                {
                  "X": 0.5620136,
                  "Y": 0.8870795
                },
                {
                  "X": 0.5615744,
                  "Y": 0.9003669
                },
                {
                  "X": 0.53482085,
                  "Y": 0.9001738
                }
              ]
            },
            "Id": "89bdc781-5a68-442d-ac54-95779d728140",
            "Page": 1,
            "Text": "at",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.14703,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017028265,
                "Left": 0.5722815,
                "Top": 0.88796645,
                "Width": 0.119822904
              },
              "Polygon": [
                {
                  "X": 0.5728159,
                  "Y": 0.88796645
                },
                {
                  "X": 0.69210434,
                  "Y": 0.88881874
                },
                {
                  "X": 0.6915687,
                  "Y": 0.9049947
                },
                {
                  "X": 0.5722815,
                  "Y": 0.904131
                }
              ]
            },
            "Id": "a0b6b8d4-9735-425a-9274-20ecf7c33189",
            "Page": 1,
            "Text": "registers",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 88.1361,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.009900276,
                "Left": 0.1206083,
                "Top": 0.8975809,
                "Width": 0.03959146
              },
              "Polygon": [
                {
                  "X": 0.12092409,
                  "Y": 0.8975809
                },
                {
                  "X": 0.16019976,
                  "Y": 0.8978646
                },
                {
                  "X": 0.15988371,
                  "Y": 0.9074812
                },
                {
                  "X": 0.1206083,
                  "Y": 0.9071954
                }
              ]
            },
            "Id": "c29a811a-cd28-4f85-a726-5b11cb5a78a7",
            "Page": 1,
            "Text": "***",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.82537,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015929956,
                "Left": 0.16988277,
                "Top": 0.89629203,
                "Width": 0.07938015
              },
              "Polygon": [
                {
                  "X": 0.17038745,
                  "Y": 0.89629203
                },
                {
                  "X": 0.24926291,
                  "Y": 0.89686084
                },
                {
                  "X": 0.24875742,
                  "Y": 0.912222
                },
                {
                  "X": 0.16988277,
                  "Y": 0.911646
                }
              ]
            },
            "Id": "67038926-23d7-4ce2-82c4-d84ec1ac4de9",
            "Page": 1,
            "Text": "Please",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.87154,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013564479,
                "Left": 0.2587759,
                "Top": 0.8997147,
                "Width": 0.029100953
              },
              "Polygon": [
                {
                  "X": 0.2592154,
                  "Y": 0.8997147
                },
                {
                  "X": 0.28787684,
                  "Y": 0.89992183
                },
                {
                  "X": 0.28743705,
                  "Y": 0.9132792
                },
                {
                  "X": 0.2587759,
                  "Y": 0.9130697
                }
              ]
            },
            "Id": "be2a2127-831a-44a7-9d8a-3a1b1f161046",
            "Page": 1,
            "Text": "be",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 67.73231,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015155664,
                "Left": 0.29919764,
                "Top": 0.89860344,
                "Width": 0.10040038
              },
              "Polygon": [
                {
                  "X": 0.29967272,
                  "Y": 0.89860344
                },
                {
                  "X": 0.39959803,
                  "Y": 0.8993249
                },
                {
                  "X": 0.399122,
                  "Y": 0.9137591
                },
                {
                  "X": 0.29919764,
                  "Y": 0.91302913
                }
              ]
            },
            "Id": "d11b2f4a-16a7-4278-8ffd-5a373a9ad786",
            "Page": 1,
            "Text": "advised.",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.6198,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015437966,
                "Left": 0.41153362,
                "Top": 0.8986622,
                "Width": 0.12045986
              },
              "Polygon": [
                {
                  "X": 0.41201395,
                  "Y": 0.8986622
                },
                {
                  "X": 0.5319935,
                  "Y": 0.8995279
                },
                {
                  "X": 0.531512,
                  "Y": 0.91410017
                },
                {
                  "X": 0.41153362,
                  "Y": 0.91322416
                }
              ]
            },
            "Id": "6c053339-5ccf-406a-8b9d-d8a4a80b0f9d",
            "Page": 1,
            "Text": "effective",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.7364,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014687773,
                "Left": 0.5406845,
                "Top": 0.90148884,
                "Width": 0.06578708
              },
              "Polygon": [
                {
                  "X": 0.54115415,
                  "Y": 0.90148884
                },
                {
                  "X": 0.6064716,
                  "Y": 0.90196085
                },
                {
                  "X": 0.6060014,
                  "Y": 0.9161766
                },
                {
                  "X": 0.5406845,
                  "Y": 0.9156991
                }
              ]
            },
            "Id": "7643dd66-9720-427e-b464-7e6d495daa7d",
            "Page": 1,
            "Text": "April",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 61.455368,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.017238775,
                "Left": 0.61940116,
                "Top": 0.9025549,
                "Width": 0.062351935
              },
              "Polygon": [
                {
                  "X": 0.6199565,
                  "Y": 0.9025549
                },
                {
                  "X": 0.6817531,
                  "Y": 0.90300167
                },
                {
                  "X": 0.68119705,
                  "Y": 0.9197937
                },
                {
                  "X": 0.61940116,
                  "Y": 0.9193408
                }
              ]
            },
            "Id": "4c9fcb83-aa03-42e9-8f55-c5c16f6e97f3",
            "Page": 1,
            "Text": "15th.",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.95134,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016022028,
                "Left": 0.695666,
                "Top": 0.90430117,
                "Width": 0.052061163
              },
              "Polygon": [
                {
                  "X": 0.69618416,
                  "Y": 0.90430117
                },
                {
                  "X": 0.74772716,
                  "Y": 0.9046742
                },
                {
                  "X": 0.7472085,
                  "Y": 0.9203232
                },
                {
                  "X": 0.695666,
                  "Y": 0.9199454
                }
              ]
            },
            "Id": "4d03bd7d-cf3e-449d-88f8-6b91526f6771",
            "Page": 1,
            "Text": "2021",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.614006,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015639002,
                "Left": 0.09846596,
                "Top": 0.9099936,
                "Width": 0.10661795
              },
              "Polygon": [
                {
                  "X": 0.098953746,
                  "Y": 0.9099936
                },
                {
                  "X": 0.2050839,
                  "Y": 0.9107679
                },
                {
                  "X": 0.20459506,
                  "Y": 0.9256326
                },
                {
                  "X": 0.09846596,
                  "Y": 0.9248489
                }
              ]
            },
            "Id": "8aa4eba9-6aa6-4f09-94da-5e0aef110251",
            "Page": 1,
            "Text": "Michaels",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.82668,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013926812,
                "Left": 0.2131285,
                "Top": 0.913118,
                "Width": 0.054576397
              },
              "Polygon": [
                {
                  "X": 0.21357341,
                  "Y": 0.913118
                },
                {
                  "X": 0.2677049,
                  "Y": 0.91351366
                },
                {
                  "X": 0.2672595,
                  "Y": 0.9270448
                },
                {
                  "X": 0.2131285,
                  "Y": 0.9266448
                }
              ]
            },
            "Id": "d00cefb9-c73e-4bb3-8cb7-3765c140f9e1",
            "Page": 1,
            "Text": "will",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.697556,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013102583,
                "Left": 0.27725333,
                "Top": 0.9142742,
                "Width": 0.028812297
              },
              "Polygon": [
                {
                  "X": 0.27767777,
                  "Y": 0.9142742
                },
                {
                  "X": 0.30606565,
                  "Y": 0.9144818
                },
                {
                  "X": 0.30564097,
                  "Y": 0.9273768
                },
                {
                  "X": 0.27725333,
                  "Y": 0.927167
                }
              ]
            },
            "Id": "6b9a2de0-a2b4-4329-b770-9246f3c5659e",
            "Page": 1,
            "Text": "be",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.67495,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014409603,
                "Left": 0.31620803,
                "Top": 0.91461384,
                "Width": 0.0806663
              },
              "Polygon": [
                {
                  "X": 0.31666315,
                  "Y": 0.91461384
                },
                {
                  "X": 0.39687434,
                  "Y": 0.91520053
                },
                {
                  "X": 0.39641848,
                  "Y": 0.9290235
                },
                {
                  "X": 0.31620803,
                  "Y": 0.92843026
                }
              ]
            },
            "Id": "469807e2-04ec-4841-aba2-478ec44ef7df",
            "Page": 1,
            "Text": "moving",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 97.95694,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015179568,
                "Left": 0.4070829,
                "Top": 0.91299963,
                "Width": 0.053979322
              },
              "Polygon": [
                {
                  "X": 0.4075705,
                  "Y": 0.91299963
                },
                {
                  "X": 0.46106222,
                  "Y": 0.91339016
                },
                {
                  "X": 0.46057406,
                  "Y": 0.9281792
                },
                {
                  "X": 0.4070829,
                  "Y": 0.927784
                }
              ]
            },
            "Id": "1287fad6-db8a-4f38-bb1b-1d70ca2436bb",
            "Page": 1,
            "Text": "from",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 39.01844,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.010042052,
                "Left": 0.47050676,
                "Top": 0.9176832,
                "Width": 0.015656836
              },
              "Polygon": [
                {
                  "X": 0.47083452,
                  "Y": 0.9176832
                },
                {
                  "X": 0.4861636,
                  "Y": 0.9177955
                },
                {
                  "X": 0.4858357,
                  "Y": 0.9277252
                },
                {
                  "X": 0.47050676,
                  "Y": 0.927612
                }
              ]
            },
            "Id": "ff70da2c-6041-46f1-9e34-d62245bc5697",
            "Page": 1,
            "Text": "a",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.82064,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014870837,
                "Left": 0.49659666,
                "Top": 0.91424906,
                "Width": 0.04116206
              },
              "Polygon": [
                {
                  "X": 0.49707785,
                  "Y": 0.91424906
                },
                {
                  "X": 0.5377587,
                  "Y": 0.9145462
                },
                {
                  "X": 0.53727716,
                  "Y": 0.9291199
                },
                {
                  "X": 0.49659666,
                  "Y": 0.92881924
                }
              ]
            },
            "Id": "0ef7c948-283a-4d38-aeb9-1710340bbb8e",
            "Page": 1,
            "Text": "180",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.89711,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01577557,
                "Left": 0.5466847,
                "Top": 0.9167544,
                "Width": 0.042437624
              },
              "Polygon": [
                {
                  "X": 0.5471958,
                  "Y": 0.9167544
                },
                {
                  "X": 0.5891223,
                  "Y": 0.9170612
                },
                {
                  "X": 0.58861077,
                  "Y": 0.93253
                },
                {
                  "X": 0.5466847,
                  "Y": 0.9322194
                }
              ]
            },
            "Id": "879ae49f-7c66-4099-875a-39e7142410c0",
            "Page": 1,
            "Text": "day",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.89689,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015536224,
                "Left": 0.59720635,
                "Top": 0.9184467,
                "Width": 0.081339665
              },
              "Polygon": [
                {
                  "X": 0.59770036,
                  "Y": 0.9184467
                },
                {
                  "X": 0.678546,
                  "Y": 0.91903883
                },
                {
                  "X": 0.67805123,
                  "Y": 0.9339829
                },
                {
                  "X": 0.59720635,
                  "Y": 0.93338364
                }
              ]
            },
            "Id": "fcbf3910-ecef-49a5-88c1-4f0d2365ed75",
            "Page": 1,
            "Text": "return",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 94.729866,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016773,
                "Left": 0.68700904,
                "Top": 0.92065597,
                "Width": 0.08270151
              },
              "Polygon": [
                {
                  "X": 0.6875443,
                  "Y": 0.92065597
                },
                {
                  "X": 0.76971054,
                  "Y": 0.92125857
                },
                {
                  "X": 0.76917446,
                  "Y": 0.93742895
                },
                {
                  "X": 0.68700904,
                  "Y": 0.93681854
                }
              ]
            },
            "Id": "1ef3037f-1027-4ae9-b55c-2c1a836363c1",
            "Page": 1,
            "Text": "policy",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 96.23188,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013546101,
                "Left": 0.09963681,
                "Top": 0.925848,
                "Width": 0.028676499
              },
              "Polygon": [
                {
                  "X": 0.100074686,
                  "Y": 0.925848
                },
                {
                  "X": 0.1283133,
                  "Y": 0.9260567
                },
                {
                  "X": 0.12787516,
                  "Y": 0.93939406
                },
                {
                  "X": 0.09963681,
                  "Y": 0.9391832
                }
              ]
            },
            "Id": "9f251528-dd18-4a24-b4e9-6e3ae13d6c63",
            "Page": 1,
            "Text": "to",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 75.639465,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01049247,
                "Left": 0.13768288,
                "Top": 0.9282366,
                "Width": 0.01607722
              },
              "Polygon": [
                {
                  "X": 0.13802376,
                  "Y": 0.9282366
                },
                {
                  "X": 0.1537601,
                  "Y": 0.92835313
                },
                {
                  "X": 0.15341914,
                  "Y": 0.9387291
                },
                {
                  "X": 0.13768288,
                  "Y": 0.9386116
                }
              ]
            },
            "Id": "91b96ec5-d9b6-4367-9324-a23b142bea26",
            "Page": 1,
            "Text": "a",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.38024,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.013433,
                "Left": 0.16109222,
                "Top": 0.92603016,
                "Width": 0.030686734
              },
              "Polygon": [
                {
                  "X": 0.1615263,
                  "Y": 0.92603016
                },
                {
                  "X": 0.19177896,
                  "Y": 0.9262537
                },
                {
                  "X": 0.19134463,
                  "Y": 0.93946314
                },
                {
                  "X": 0.16109222,
                  "Y": 0.93923724
                }
              ]
            },
            "Id": "52535c19-5232-4f15-ae75-1d1fac825e67",
            "Page": 1,
            "Text": "60",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 84.99701,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015225087,
                "Left": 0.20042342,
                "Top": 0.9270765,
                "Width": 0.04249108
              },
              "Polygon": [
                {
                  "X": 0.20091377,
                  "Y": 0.9270765
                },
                {
                  "X": 0.2429145,
                  "Y": 0.92738706
                },
                {
                  "X": 0.24242374,
                  "Y": 0.94230163
                },
                {
                  "X": 0.20042342,
                  "Y": 0.9419874
                }
              ]
            },
            "Id": "e1bacdff-c199-4d51-948b-1d3701d286e3",
            "Page": 1,
            "Text": "day",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.72207,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014255205,
                "Left": 0.2509061,
                "Top": 0.92836773,
                "Width": 0.08071709
              },
              "Polygon": [
                {
                  "X": 0.25135547,
                  "Y": 0.92836773
                },
                {
                  "X": 0.3316232,
                  "Y": 0.9289616
                },
                {
                  "X": 0.3311731,
                  "Y": 0.94262296
                },
                {
                  "X": 0.2509061,
                  "Y": 0.9420226
                }
              ]
            },
            "Id": "72ce001b-1a4a-4784-a515-bb546323d1e8",
            "Page": 1,
            "Text": "return",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.56359,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015601887,
                "Left": 0.33985412,
                "Top": 0.92764014,
                "Width": 0.082438275
              },
              "Polygon": [
                {
                  "X": 0.34034804,
                  "Y": 0.92764014
                },
                {
                  "X": 0.4222924,
                  "Y": 0.9282457
                },
                {
                  "X": 0.4217977,
                  "Y": 0.943242
                },
                {
                  "X": 0.33985412,
                  "Y": 0.94262916
                }
              ]
            },
            "Id": "6b53904d-c14c-4e9c-81dc-a050866ac522",
            "Page": 1,
            "Text": "policy",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 93.011604,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.015637007,
                "Left": 0.4318092,
                "Top": 0.9274963,
                "Width": 0.05433709
              },
              "Polygon": [
                {
                  "X": 0.43231183,
                  "Y": 0.9274963
                },
                {
                  "X": 0.48614627,
                  "Y": 0.92789394
                },
                {
                  "X": 0.48564306,
                  "Y": 0.94313335
                },
                {
                  "X": 0.4318092,
                  "Y": 0.9427309
                }
              ]
            },
            "Id": "4c68d767-0e16-4823-a147-0fa1bc5ffb18",
            "Page": 1,
            "Text": "from",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.84294,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014786981,
                "Left": 0.4974668,
                "Top": 0.9297179,
                "Width": 0.03857981
              },
              "Polygon": [
                {
                  "X": 0.49794576,
                  "Y": 0.9297179
                },
                {
                  "X": 0.5360466,
                  "Y": 0.92999965
                },
                {
                  "X": 0.5355673,
                  "Y": 0.94450486
                },
                {
                  "X": 0.4974668,
                  "Y": 0.9442198
                }
              ]
            },
            "Id": "b963e3ee-76eb-401b-a1fb-ac8ca60ac6dd",
            "Page": 1,
            "Text": "the",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.751205,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0152367875,
                "Left": 0.54676455,
                "Top": 0.9315364,
                "Width": 0.05385647
              },
              "Polygon": [
                {
                  "X": 0.5472549,
                  "Y": 0.9315364
                },
                {
                  "X": 0.600621,
                  "Y": 0.9319315
                },
                {
                  "X": 0.60013014,
                  "Y": 0.9467732
                },
                {
                  "X": 0.54676455,
                  "Y": 0.94637334
                }
              ]
            },
            "Id": "0fab1758-c52b-49fe-b438-1df19724cf61",
            "Page": 1,
            "Text": "date",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.75443,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014453369,
                "Left": 0.61007124,
                "Top": 0.93315166,
                "Width": 0.027741428
              },
              "Polygon": [
                {
                  "X": 0.6105426,
                  "Y": 0.93315166
                },
                {
                  "X": 0.6378127,
                  "Y": 0.9333537
                },
                {
                  "X": 0.637341,
                  "Y": 0.947605
                },
                {
                  "X": 0.61007124,
                  "Y": 0.9474006
                }
              ]
            },
            "Id": "4c044393-4c0e-4c0e-bfa8-c193469cc948",
            "Page": 1,
            "Text": "of",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 73.352844,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016068818,
                "Left": 0.64848703,
                "Top": 0.9355415,
                "Width": 0.11032472
              },
              "Polygon": [
                {
                  "X": 0.6489916,
                  "Y": 0.9355415
                },
                {
                  "X": 0.7588118,
                  "Y": 0.9363568
                },
                {
                  "X": 0.7583061,
                  "Y": 0.9516103
                },
                {
                  "X": 0.64848703,
                  "Y": 0.95078516
                }
              ]
            },
            "Id": "27c593f6-f9e8-4b49-b6a3-ff87d9cd3a85",
            "Page": 1,
            "Text": "purchase",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.65105,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01571053,
                "Left": 0.10956757,
                "Top": 0.9394749,
                "Width": 0.07978925
              },
              "Polygon": [
                {
                  "X": 0.11006384,
                  "Y": 0.9394749
                },
                {
                  "X": 0.18935682,
                  "Y": 0.94006723
                },
                {
                  "X": 0.18885975,
                  "Y": 0.9551854
                },
                {
                  "X": 0.10956757,
                  "Y": 0.9545859
                }
              ]
            },
            "Id": "883591b3-376e-46aa-9ae3-ad5d48b393b9",
            "Page": 1,
            "Text": "Please",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.556206,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.011419082,
                "Left": 0.20014037,
                "Top": 0.94410694,
                "Width": 0.041813802
              },
              "Polygon": [
                {
                  "X": 0.20050558,
                  "Y": 0.94410694
                },
                {
                  "X": 0.24195418,
                  "Y": 0.94441754
                },
                {
                  "X": 0.24158867,
                  "Y": 0.955526
                },
                {
                  "X": 0.20014037,
                  "Y": 0.95521265
                }
              ]
            },
            "Id": "1daabd12-3551-4db4-ab30-61344cccab11",
            "Page": 1,
            "Text": "see",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 94.54862,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.009731145,
                "Left": 0.25214794,
                "Top": 0.9451593,
                "Width": 0.015727185
              },
              "Polygon": [
                {
                  "X": 0.25246435,
                  "Y": 0.9451593
                },
                {
                  "X": 0.26787513,
                  "Y": 0.9452749
                },
                {
                  "X": 0.2675586,
                  "Y": 0.9548905
                },
                {
                  "X": 0.25214794,
                  "Y": 0.9547741
                }
              ]
            },
            "Id": "17ba67f9-4101-4e0d-96f8-7f23afacb50e",
            "Page": 1,
            "Text": "a",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.66611,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014224538,
                "Left": 0.27797735,
                "Top": 0.9422416,
                "Width": 0.066649556
              },
              "Polygon": [
                {
                  "X": 0.27842915,
                  "Y": 0.9422416
                },
                {
                  "X": 0.3446269,
                  "Y": 0.94273674
                },
                {
                  "X": 0.34417447,
                  "Y": 0.95646614
                },
                {
                  "X": 0.27797735,
                  "Y": 0.95596564
                }
              ]
            },
            "Id": "21635901-0ce7-4f23-b2b7-100d19d84744",
            "Page": 1,
            "Text": "store",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.2162,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01568951,
                "Left": 0.35364595,
                "Top": 0.9424362,
                "Width": 0.11820034
              },
              "Polygon": [
                {
                  "X": 0.35413367,
                  "Y": 0.9424362
                },
                {
                  "X": 0.47184628,
                  "Y": 0.94331634
                },
                {
                  "X": 0.47135738,
                  "Y": 0.9581257
                },
                {
                  "X": 0.35364595,
                  "Y": 0.9572352
                }
              ]
            },
            "Id": "b6df18c5-354b-48fe-bf68-07ef1f47dbb4",
            "Page": 1,
            "Text": "associate",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 98.03991,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.014996213,
                "Left": 0.48145348,
                "Top": 0.94396925,
                "Width": 0.041052822
              },
              "Polygon": [
                {
                  "X": 0.48193848,
                  "Y": 0.94396925
                },
                {
                  "X": 0.5225063,
                  "Y": 0.94427276
                },
                {
                  "X": 0.5220209,
                  "Y": 0.9589655
                },
                {
                  "X": 0.48145348,
                  "Y": 0.95865846
                }
              ]
            },
            "Id": "598f136b-226b-4337-9855-00462374dcdb",
            "Page": 1,
            "Text": "for",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 91.82618,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.0127951205,
                "Left": 0.5324079,
                "Top": 0.9481637,
                "Width": 0.054206
              },
              "Polygon": [
                {
                  "X": 0.5328172,
                  "Y": 0.9481637
                },
                {
                  "X": 0.5866139,
                  "Y": 0.94856733
                },
                {
                  "X": 0.5862041,
                  "Y": 0.96095884
                },
                {
                  "X": 0.5324079,
                  "Y": 0.9605512
                }
              ]
            },
            "Id": "b5ad6bc8-e286-4e92-8e6d-62e06c6a542f",
            "Page": 1,
            "Text": "more",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 83.36131,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016617654,
                "Left": 0.6009797,
                "Top": 0.94868714,
                "Width": 0.14564608
              },
              "Polygon": [
                {
                  "X": 0.6014929,
                  "Y": 0.94868714
                },
                {
                  "X": 0.7466258,
                  "Y": 0.9497761
                },
                {
                  "X": 0.7461111,
                  "Y": 0.9653048
                },
                {
                  "X": 0.6009797,
                  "Y": 0.96420246
                }
              ]
            },
            "Id": "1da2823c-c7c5-4ffc-a8de-bb5f8ba2e236",
            "Page": 1,
            "Text": "information",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.26125,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.016906206,
                "Left": 0.38011658,
                "Top": 0.9683744,
                "Width": 0.09147111
              },
              "Polygon": [
                {
                  "X": 0.38065085,
                  "Y": 0.9683744
                },
                {
                  "X": 0.47158772,
                  "Y": 0.9690682
                },
                {
                  "X": 0.47105247,
                  "Y": 0.9852806
                },
                {
                  "X": 0.38011658,
                  "Y": 0.984578
                }
              ]
            },
            "Id": "ca91f92d-a2e3-4bc2-828a-2238c48eb50a",
            "Page": 1,
            "Text": "1/04/22",
            "TextType": "PRINTED"
          },
          {
            "BlockType": "WORD",
            "Confidence": 99.33398,
            "Geometry": {
              "BoundingBox": {
                "Height": 0.01696282,
                "Left": 0.4939228,
                "Top": 0.972132,
                "Width": 0.0663983
              },
              "Polygon": [
                {
                  "X": 0.49446613,
                  "Y": 0.972132
                },
                {
                  "X": 0.5603211,
                  "Y": 0.9726356
                },
                {
                  "X": 0.559777,
                  "Y": 0.98909485
                },
                {
                  "X": 0.4939228,
                  "Y": 0.9885848
                }
              ]
            },
            "Id": "087a26bf-289e-4f81-b9ba-446f723efc46",
            "Page": 1,
            "Text": "13:03",
            "TextType": "PRINTED"
          }
        ],
        "ExpenseIndex": 1,
        "LineItemGroups": [
          {
            "LineItemGroupIndex": 1,
            "LineItems": [
              {
                "LineItemExpenseFields": [
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 91.53479,
                      "Text": "PRODUCT_CODE"
                    },
                    "ValueDetection": {
                      "Confidence": 91.41411,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.016323432,
                          "Left": 0.37915742,
                          "Top": 0.29163498,
                          "Width": 0.15397784
                        },
                        "Polygon": [
                          {
                            "X": 0.37967667,
                            "Y": 0.29163498
                          },
                          {
                            "X": 0.53313524,
                            "Y": 0.29218993
                          },
                          {
                            "X": 0.53261435,
                            "Y": 0.3079584
                          },
                          {
                            "X": 0.37915742,
                            "Y": 0.3073891
                          }
                        ]
                      },
                      "Text": "647658036793"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 99.543,
                      "Text": "ITEM"
                    },
                    "ValueDetection": {
                      "Confidence": 99.20062,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.019060778,
                          "Left": 0.123208396,
                          "Top": 0.2856913,
                          "Width": 0.22367601
                        },
                        "Polygon": [
                          {
                            "X": 0.123807244,
                            "Y": 0.2856913
                          },
                          {
                            "X": 0.3468844,
                            "Y": 0.28649136
                          },
                          {
                            "X": 0.34628284,
                            "Y": 0.30475205
                          },
                          {
                            "X": 0.123208396,
                            "Y": 0.30392784
                          }
                        ]
                      },
                      "Text": "CNCL 2002 HOLIDAY"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 99.499245,
                      "Text": "PRICE"
                    },
                    "ValueDetection": {
                      "Confidence": 99.487785,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.017870836,
                          "Left": 0.6894336,
                          "Top": 0.2988097,
                          "Width": 0.05835685
                        },
                        "Polygon": [
                          {
                            "X": 0.69001806,
                            "Y": 0.2988097
                          },
                          {
                            "X": 0.74779046,
                            "Y": 0.2990207
                          },
                          {
                            "X": 0.74720526,
                            "Y": 0.31668055
                          },
                          {
                            "X": 0.6894336,
                            "Y": 0.3164635
                          }
                        ]
                      },
                      "Text": "6.00"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 93.92119,
                      "Text": "UNIT_PRICE"
                    },
                    "ValueDetection": {
                      "Confidence": 92.798546,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.015772834,
                          "Left": 0.47831848,
                          "Top": 0.30589336,
                          "Width": 0.07772607
                        },
                        "Polygon": [
                          {
                            "X": 0.4788294,
                            "Y": 0.30589336
                          },
                          {
                            "X": 0.5560445,
                            "Y": 0.30617896
                          },
                          {
                            "X": 0.5555328,
                            "Y": 0.3216662
                          },
                          {
                            "X": 0.47831848,
                            "Y": 0.32137352
                          }
                        ]
                      },
                      "Text": "6.00"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 91.68216,
                      "Text": "EXPENSE_ROW"
                    },
                    "ValueDetection": {
                      "Confidence": 91.39553,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.0368461,
                          "Left": 0.12267978,
                          "Top": 0.2856913,
                          "Width": 0.6638308
                        },
                        "Polygon": [
                          {
                            "X": 0.123807244,
                            "Y": 0.2856913
                          },
                          {
                            "X": 0.7865106,
                            "Y": 0.2880681
                          },
                          {
                            "X": 0.78536785,
                            "Y": 0.3225374
                          },
                          {
                            "X": 0.12267978,
                            "Y": 0.32002547
                          }
                        ]
                      },
                      "Text": "CNCL 2002 HOLIDAY 647658036793 19.99 6.00 P\n6.00"
                    }
                  }
                ]
              },
              {
                "LineItemExpenseFields": [
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 91.70498,
                      "Text": "PRODUCT_CODE"
                    },
                    "ValueDetection": {
                      "Confidence": 91.693275,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.01587738,
                          "Left": 0.38050675,
                          "Top": 0.31981584,
                          "Width": 0.15266068
                        },
                        "Polygon": [
                          {
                            "X": 0.38101065,
                            "Y": 0.31981584
                          },
                          {
                            "X": 0.5331674,
                            "Y": 0.3203915
                          },
                          {
                            "X": 0.532662,
                            "Y": 0.3356932
                          },
                          {
                            "X": 0.38050675,
                            "Y": 0.33510375
                          }
                        ]
                      },
                      "Text": "192040076524"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 99.631676,
                      "Text": "ITEM"
                    },
                    "ValueDetection": {
                      "Confidence": 99.10727,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.019568311,
                          "Left": 0.12374012,
                          "Top": 0.31482646,
                          "Width": 0.22283494
                        },
                        "Polygon": [
                          {
                            "X": 0.124354474,
                            "Y": 0.31482646
                          },
                          {
                            "X": 0.34657508,
                            "Y": 0.3156619
                          },
                          {
                            "X": 0.34595793,
                            "Y": 0.33439475
                          },
                          {
                            "X": 0.12374012,
                            "Y": 0.33353463
                          }
                        ]
                      },
                      "Text": "ST TREND STYLE PH"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 95.89493,
                      "Text": "PRICE"
                    },
                    "ValueDetection": {
                      "Confidence": 95.84593,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.016036743,
                          "Left": 0.6884606,
                          "Top": 0.33004457,
                          "Width": 0.058511678
                        },
                        "Polygon": [
                          {
                            "X": 0.688984,
                            "Y": 0.33004457
                          },
                          {
                            "X": 0.74697226,
                            "Y": 0.33026707
                          },
                          {
                            "X": 0.7464482,
                            "Y": 0.34608132
                          },
                          {
                            "X": 0.6884606,
                            "Y": 0.3458534
                          }
                        ]
                      },
                      "Text": "5.98"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 59.94398,
                      "Text": "UNIT_PRICE"
                    },
                    "ValueDetection": {
                      "Confidence": 59.231834,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.017015213,
                          "Left": 0.45095524,
                          "Top": 0.33316743,
                          "Width": 0.10565274
                        },
                        "Polygon": [
                          {
                            "X": 0.45150292,
                            "Y": 0.33316743
                          },
                          {
                            "X": 0.55660796,
                            "Y": 0.33357325
                          },
                          {
                            "X": 0.5560591,
                            "Y": 0.35018265
                          },
                          {
                            "X": 0.45095524,
                            "Y": 0.3497665
                          }
                        ]
                      },
                      "Text": "2@2.99"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 92.12048,
                      "Text": "EXPENSE_ROW"
                    },
                    "ValueDetection": {
                      "Confidence": 91.754265,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.036262523,
                          "Left": 0.123249695,
                          "Top": 0.31482646,
                          "Width": 0.6628316
                        },
                        "Polygon": [
                          {
                            "X": 0.124354474,
                            "Y": 0.31482646
                          },
                          {
                            "X": 0.78608125,
                            "Y": 0.31731427
                          },
                          {
                            "X": 0.7849615,
                            "Y": 0.35108897
                          },
                          {
                            "X": 0.123249695,
                            "Y": 0.34846896
                          }
                        ]
                      },
                      "Text": "ST TREND STYLE PH 192040076524 5.99\n5.98 P\n2@2.99"
                    }
                  }
                ]
              },
              {
                "LineItemExpenseFields": [
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 92.624794,
                      "Text": "PRODUCT_CODE"
                    },
                    "ValueDetection": {
                      "Confidence": 92.60293,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.016056914,
                          "Left": 0.37939972,
                          "Top": 0.3496506,
                          "Width": 0.15037706
                        },
                        "Polygon": [
                          {
                            "X": 0.37990895,
                            "Y": 0.3496506
                          },
                          {
                            "X": 0.5297768,
                            "Y": 0.35024413
                          },
                          {
                            "X": 0.529266,
                            "Y": 0.36570752
                          },
                          {
                            "X": 0.37939972,
                            "Y": 0.36510023
                          }
                        ]
                      },
                      "Text": "T29911060087"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 99.78426,
                      "Text": "ITEM"
                    },
                    "ValueDetection": {
                      "Confidence": 99.65301,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.019287316,
                          "Left": 0.12272784,
                          "Top": 0.34596428,
                          "Width": 0.2204092
                        },
                        "Polygon": [
                          {
                            "X": 0.12333195,
                            "Y": 0.34596428
                          },
                          {
                            "X": 0.34313703,
                            "Y": 0.34683132
                          },
                          {
                            "X": 0.34253022,
                            "Y": 0.3652516
                          },
                          {
                            "X": 0.12272784,
                            "Y": 0.36436054
                          }
                        ]
                      },
                      "Text": "GA LINSEED REFINE"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 99.95049,
                      "Text": "PRICE"
                    },
                    "ValueDetection": {
                      "Confidence": 99.80132,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.01638707,
                          "Left": 0.6796889,
                          "Top": 0.36025557,
                          "Width": 0.06714812
                        },
                        "Polygon": [
                          {
                            "X": 0.6802223,
                            "Y": 0.36025557
                          },
                          {
                            "X": 0.74683696,
                            "Y": 0.3605231
                          },
                          {
                            "X": 0.74630284,
                            "Y": 0.37664264
                          },
                          {
                            "X": 0.6796889,
                            "Y": 0.37636876
                          }
                        ]
                      },
                      "Text": "12.79"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 99.69892,
                      "Text": "UNIT_PRICE"
                    },
                    "ValueDetection": {
                      "Confidence": 98.63851,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.015425791,
                          "Left": 0.502107,
                          "Top": 0.3634753,
                          "Width": 0.064911716
                        },
                        "Polygon": [
                          {
                            "X": 0.5026076,
                            "Y": 0.3634753
                          },
                          {
                            "X": 0.5670187,
                            "Y": 0.3637355
                          },
                          {
                            "X": 0.5665175,
                            "Y": 0.3789011
                          },
                          {
                            "X": 0.502107,
                            "Y": 0.3786351
                          }
                        ]
                      },
                      "Text": "12.79"
                    }
                  },
                  {
                    "PageNumber": 1,
                    "Type": {
                      "Confidence": 97.552925,
                      "Text": "EXPENSE_ROW"
                    },
                    "ValueDetection": {
                      "Confidence": 82.55673,
                      "Geometry": {
                        "BoundingBox": {
                          "Height": 0.033691186,
                          "Left": 0.12231018,
                          "Top": 0.34596428,
                          "Width": 0.6249277
                        },
                        "Polygon": [
                          {
                            "X": 0.12333195,
                            "Y": 0.34596428
                          },
                          {
                            "X": 0.74723786,
                            "Y": 0.3484253
                          },
                          {
                            "X": 0.746203,
                            "Y": 0.37965545
                          },
                          {
                            "X": 0.12231018,
                            "Y": 0.37707916
                          }
                        ]
                      },
                      "Text": "GA LINSEED REFINE T29911060087 15.99\n10 12.79 12.79"
                    }
                  }
                ]
              }
            ]
          }
        ],
        "SummaryFields": [
          {
            "GroupProperties": [
              {
                "Id": "aa042294-e375-4bf6-b10e-d1f5aa2224ca",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.172745,
              "Text": "ADDRESS"
            },
            "ValueDetection": {
              "Confidence": 82.233574,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.0736636,
                  "Left": 0.23564203,
                  "Top": 0.13027571,
                  "Width": 0.3646074
                },
                "Polygon": [
                  {
                    "X": 0.23802797,
                    "Y": 0.13027571
                  },
                  {
                    "X": 0.6002494,
                    "Y": 0.13123979
                  },
                  {
                    "X": 0.5978458,
                    "Y": 0.2039393
                  },
                  {
                    "X": 0.23564203,
                    "Y": 0.2028193
                  }
                ]
              },
              "Text": "MICHAELS STORE #9010 (386)\nMICHAELS STORE #9010\n5507 S WILL IAMSON BLVD\nPORT ORGANCE FL 0328\nRewards"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "aa042294-e375-4bf6-b10e-d1f5aa2224ca",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.172745,
              "Text": "STREET"
            },
            "ValueDetection": {
              "Confidence": 62.94082,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.02126451,
                  "Left": 0.32004637,
                  "Top": 0.1586597,
                  "Width": 0.2742566
                },
                "Polygon": [
                  {
                    "X": 0.32072002,
                    "Y": 0.1586597
                  },
                  {
                    "X": 0.59430295,
                    "Y": 0.15943356
                  },
                  {
                    "X": 0.59362555,
                    "Y": 0.1799242
                  },
                  {
                    "X": 0.32004637,
                    "Y": 0.17911713
                  }
                ]
              },
              "Text": "5507 S WILL IAMSON BLVD"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "aa042294-e375-4bf6-b10e-d1f5aa2224ca",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.172745,
              "Text": "CITY"
            },
            "ValueDetection": {
              "Confidence": 97.83043,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.020966168,
                  "Left": 0.3180413,
                  "Top": 0.17374146,
                  "Width": 0.15835741
                },
                "Polygon": [
                  {
                    "X": 0.3187159,
                    "Y": 0.17374146
                  },
                  {
                    "X": 0.4763987,
                    "Y": 0.17420161
                  },
                  {
                    "X": 0.47572193,
                    "Y": 0.19470763
                  },
                  {
                    "X": 0.3180413,
                    "Y": 0.19422832
                  }
                ]
              },
              "Text": "PORT\nORGANCE"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "aa042294-e375-4bf6-b10e-d1f5aa2224ca",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.172745,
              "Text": "STATE"
            },
            "ValueDetection": {
              "Confidence": 57.359333,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.013183529,
                  "Left": 0.49091747,
                  "Top": 0.17827266,
                  "Width": 0.032156836
                },
                "Polygon": [
                  {
                    "X": 0.49134952,
                    "Y": 0.17827266
                  },
                  {
                    "X": 0.5230743,
                    "Y": 0.17836599
                  },
                  {
                    "X": 0.52264196,
                    "Y": 0.19145618
                  },
                  {
                    "X": 0.49091747,
                    "Y": 0.19136038
                  }
                ]
              },
              "Text": "FL"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "aa042294-e375-4bf6-b10e-d1f5aa2224ca",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.172745,
              "Text": "ZIP_CODE"
            },
            "ValueDetection": {
              "Confidence": 49.08386,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.016288899,
                  "Left": 0.53180814,
                  "Top": 0.17235066,
                  "Width": 0.059487913
                },
                "Polygon": [
                  {
                    "X": 0.5323403,
                    "Y": 0.17235066
                  },
                  {
                    "X": 0.591296,
                    "Y": 0.172522
                  },
                  {
                    "X": 0.5907632,
                    "Y": 0.18863957
                  },
                  {
                    "X": 0.53180814,
                    "Y": 0.1884626
                  }
                ]
              },
              "Text": "0328"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "aa042294-e375-4bf6-b10e-d1f5aa2224ca",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.172745,
              "Text": "ADDRESS_BLOCK"
            },
            "ValueDetection": {
              "Confidence": 74.65822,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.047849014,
                  "Left": 0.3180413,
                  "Top": 0.14721549,
                  "Width": 0.2766405
                },
                "Polygon": [
                  {
                    "X": 0.31958938,
                    "Y": 0.14721549
                  },
                  {
                    "X": 0.5946818,
                    "Y": 0.14797495
                  },
                  {
                    "X": 0.593125,
                    "Y": 0.1950645
                  },
                  {
                    "X": 0.3180413,
                    "Y": 0.19422832
                  }
                ]
              },
              "Text": "MICHAELS STORE #9010\n5507 S WILL IAMSON BLVD\nPORT 0328\nORGANCE FL"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "771f6ec3-b586-4e44-ba1c-f9eaabbacd0f",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.82914,
              "Text": "NAME"
            },
            "ValueDetection": {
              "Confidence": 99.774445,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015820362,
                  "Left": 0.43053368,
                  "Top": 0.77923393,
                  "Width": 0.10620486
                },
                "Polygon": [
                  {
                    "X": 0.43103263,
                    "Y": 0.77923393
                  },
                  {
                    "X": 0.5367385,
                    "Y": 0.7799217
                  },
                  {
                    "X": 0.53623855,
                    "Y": 0.7950543
                  },
                  {
                    "X": 0.43053368,
                    "Y": 0.79435706
                  }
                ]
              },
              "Text": "Michaels"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "8cbf0984-aab4-4a1b-bbdf-1fd3e5bf8aa3",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.261604,
              "Text": "NAME"
            },
            "ValueDetection": {
              "Confidence": 99.254166,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.017535156,
                  "Left": 0.34749302,
                  "Top": 0.74923056,
                  "Width": 0.18318836
                },
                "Polygon": [
                  {
                    "X": 0.34803215,
                    "Y": 0.74923056
                  },
                  {
                    "X": 0.5306814,
                    "Y": 0.750387
                  },
                  {
                    "X": 0.5301402,
                    "Y": 0.7667657
                  },
                  {
                    "X": 0.34749302,
                    "Y": 0.76559156
                  }
                ]
              },
              "Text": "Baron Brothers"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "7a77aedb-72c0-405f-b047-1b24a980b05d",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 84.58116,
              "Text": "NAME"
            },
            "ValueDetection": {
              "Confidence": 84.57034,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.016403547,
                  "Left": 0.09846482,
                  "Top": 0.9092639,
                  "Width": 0.1066431
                },
                "Polygon": [
                  {
                    "X": 0.09897771,
                    "Y": 0.9092639
                  },
                  {
                    "X": 0.20510791,
                    "Y": 0.91003776
                  },
                  {
                    "X": 0.20459391,
                    "Y": 0.92566746
                  },
                  {
                    "X": 0.09846482,
                    "Y": 0.9248838
                  }
                ]
              },
              "Text": "Michaels"
            }
          },
          {
            "GroupProperties": [
              {
                "Id": "15a4ed3b-127d-4527-97a4-2541adaf7c02",
                "Types": [
                  "VENDOR"
                ]
              }
            ],
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.61307,
              "Text": "NAME"
            },
            "ValueDetection": {
              "Confidence": 99.38257,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.067741856,
                  "Left": 0.27484772,
                  "Top": 0.02388506,
                  "Width": 0.37585786
                },
                "Polygon": [
                  {
                    "X": 0.27704704,
                    "Y": 0.02388506
                  },
                  {
                    "X": 0.6507056,
                    "Y": 0.024643417
                  },
                  {
                    "X": 0.6484895,
                    "Y": 0.09162691
                  },
                  {
                    "X": 0.27484772,
                    "Y": 0.09072038
                  }
                ]
              },
              "Text": "Michaels"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.71493,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.014611467,
                  "Left": 0.1518607,
                  "Top": 0.52709156,
                  "Width": 0.057538606
                },
                "Polygon": [
                  {
                    "X": 0.15233119,
                    "Y": 0.52709156
                  },
                  {
                    "X": 0.2093993,
                    "Y": 0.527378
                  },
                  {
                    "X": 0.20892826,
                    "Y": 0.541703
                  },
                  {
                    "X": 0.1518607,
                    "Y": 0.5414117
                  }
                ]
              },
              "Text": "Visa"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.74812,
              "Text": "AMOUNT_PAID"
            },
            "ValueDetection": {
              "Confidence": 99.729805,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015559327,
                  "Left": 0.5106136,
                  "Top": 0.5266225,
                  "Width": 0.06945804
                },
                "Polygon": [
                  {
                    "X": 0.51111585,
                    "Y": 0.5266225
                  },
                  {
                    "X": 0.5800716,
                    "Y": 0.52696764
                  },
                  {
                    "X": 0.5795687,
                    "Y": 0.5421818
                  },
                  {
                    "X": 0.5106136,
                    "Y": 0.54183036
                  }
                ]
              },
              "Text": "26.65"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 83.444496,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015969986,
                  "Left": 0.3772232,
                  "Top": 0.3942157,
                  "Width": 0.14058574
                },
                "Polygon": [
                  {
                    "X": 0.37772965,
                    "Y": 0.3942157
                  },
                  {
                    "X": 0.5178089,
                    "Y": 0.39480752
                  },
                  {
                    "X": 0.517301,
                    "Y": 0.4101857
                  },
                  {
                    "X": 0.3772232,
                    "Y": 0.40958112
                  }
                ]
              },
              "Text": "YOU SAVED $"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 84.87797,
              "Text": "DISCOUNT"
            },
            "ValueDetection": {
              "Confidence": 84.965355,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015759587,
                  "Left": 0.54934925,
                  "Top": 0.39337718,
                  "Width": 0.0684966
                },
                "Polygon": [
                  {
                    "X": 0.5498603,
                    "Y": 0.39337718
                  },
                  {
                    "X": 0.61784583,
                    "Y": 0.39366376
                  },
                  {
                    "X": 0.61733407,
                    "Y": 0.40913677
                  },
                  {
                    "X": 0.54934925,
                    "Y": 0.40884393
                  }
                ]
              },
              "Text": "23.19"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.510445,
              "Text": "INVOICE_RECEIPT_DATE"
            },
            "ValueDetection": {
              "Confidence": 99.46372,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.018221913,
                  "Left": 0.60258126,
                  "Top": 0.27097937,
                  "Width": 0.09301256
                },
                "Polygon": [
                  {
                    "X": 0.6031728,
                    "Y": 0.27097937
                  },
                  {
                    "X": 0.6955938,
                    "Y": 0.27130184
                  },
                  {
                    "X": 0.6950011,
                    "Y": 0.2892013
                  },
                  {
                    "X": 0.60258126,
                    "Y": 0.28886902
                  }
                ]
              },
              "Text": "1/04/22"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.8683,
              "Text": "INVOICE_RECEIPT_DATE"
            },
            "ValueDetection": {
              "Confidence": 98.85552,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.016906206,
                  "Left": 0.38011658,
                  "Top": 0.9683744,
                  "Width": 0.09147111
                },
                "Polygon": [
                  {
                    "X": 0.38065085,
                    "Y": 0.9683744
                  },
                  {
                    "X": 0.47158772,
                    "Y": 0.9690682
                  },
                  {
                    "X": 0.47105247,
                    "Y": 0.9852806
                  },
                  {
                    "X": 0.38011658,
                    "Y": 0.984578
                  }
                ]
              },
              "Text": "1/04/22"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 63.549675,
              "Text": "INVOICE_RECEIPT_ID"
            },
            "ValueDetection": {
              "Confidence": 63.54417,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.016121564,
                  "Left": 0.16527355,
                  "Top": 0.27162626,
                  "Width": 0.09301018
                },
                "Polygon": [
                  {
                    "X": 0.16579233,
                    "Y": 0.27162626
                  },
                  {
                    "X": 0.25828373,
                    "Y": 0.2719502
                  },
                  {
                    "X": 0.25776395,
                    "Y": 0.28774783
                  },
                  {
                    "X": 0.16527355,
                    "Y": 0.28741524
                  }
                ]
              },
              "Text": "4033602"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.68754,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015428133,
                  "Left": 0.2879345,
                  "Top": 0.43507072,
                  "Width": 0.1029114
                },
                "Polygon": [
                  {
                    "X": 0.28842703,
                    "Y": 0.43507072
                  },
                  {
                    "X": 0.39084592,
                    "Y": 0.4355285
                  },
                  {
                    "X": 0.39035237,
                    "Y": 0.45049885
                  },
                  {
                    "X": 0.2879345,
                    "Y": 0.450032
                  }
                ]
              },
              "Text": "SUBTOTAL"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.74496,
              "Text": "SUBTOTAL"
            },
            "ValueDetection": {
              "Confidence": 99.72074,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015480844,
                  "Left": 0.5111842,
                  "Top": 0.43558502,
                  "Width": 0.067528576
                },
                "Polygon": [
                  {
                    "X": 0.5116854,
                    "Y": 0.43558502
                  },
                  {
                    "X": 0.5787128,
                    "Y": 0.43588442
                  },
                  {
                    "X": 0.57821095,
                    "Y": 0.45106587
                  },
                  {
                    "X": 0.5111842,
                    "Y": 0.45076045
                  }
                ]
              },
              "Text": "24.77"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 55.58008,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015996022,
                  "Left": 0.20950818,
                  "Top": 0.46253878,
                  "Width": 0.1795444
                },
                "Polygon": [
                  {
                    "X": 0.21000634,
                    "Y": 0.46253878
                  },
                  {
                    "X": 0.38905257,
                    "Y": 0.4633686
                  },
                  {
                    "X": 0.3885526,
                    "Y": 0.4785348
                  },
                  {
                    "X": 0.20950818,
                    "Y": 0.47768888
                  }
                ]
              },
              "Text": "SUBTOTAL W/PIF"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 55.63153,
              "Text": "SUBTOTAL"
            },
            "ValueDetection": {
              "Confidence": 55.487747,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.014374548,
                  "Left": 0.51215535,
                  "Top": 0.46427065,
                  "Width": 0.067035526
                },
                "Polygon": [
                  {
                    "X": 0.51261973,
                    "Y": 0.46427065
                  },
                  {
                    "X": 0.5791909,
                    "Y": 0.4645793
                  },
                  {
                    "X": 0.5787259,
                    "Y": 0.4786452
                  },
                  {
                    "X": 0.51215535,
                    "Y": 0.478331
                  }
                ]
              },
              "Text": "25.02"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 78.19542,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015717698,
                  "Left": 0.2097694,
                  "Top": 0.4485956,
                  "Width": 0.1304124
                },
                "Polygon": [
                  {
                    "X": 0.2102664,
                    "Y": 0.4485956
                  },
                  {
                    "X": 0.3401818,
                    "Y": 0.44918695
                  },
                  {
                    "X": 0.33968347,
                    "Y": 0.4643133
                  },
                  {
                    "X": 0.2097694,
                    "Y": 0.4637103
                  }
                ]
              },
              "Text": "PIF 1.00 %"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 96.756294,
              "Text": "TAX"
            },
            "ValueDetection": {
              "Confidence": 84.997955,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.0139398435,
                  "Left": 0.54264784,
                  "Top": 0.45071492,
                  "Width": 0.035878778
                },
                "Polygon": [
                  {
                    "X": 0.543103,
                    "Y": 0.45071492
                  },
                  {
                    "X": 0.5785266,
                    "Y": 0.45087627
                  },
                  {
                    "X": 0.5780711,
                    "Y": 0.46465474
                  },
                  {
                    "X": 0.54264784,
                    "Y": 0.46449047
                  }
                ]
              },
              "Text": "25"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.34427,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.01639703,
                  "Left": 0.20817952,
                  "Top": 0.47725305,
                  "Width": 0.18201973
                },
                "Polygon": [
                  {
                    "X": 0.20868994,
                    "Y": 0.47725305
                  },
                  {
                    "X": 0.39019924,
                    "Y": 0.47811016
                  },
                  {
                    "X": 0.3896869,
                    "Y": 0.49365008
                  },
                  {
                    "X": 0.20817952,
                    "Y": 0.49277627
                  }
                ]
              },
              "Text": "Sales Tax 6.5%"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.75611,
              "Text": "TAX"
            },
            "ValueDetection": {
              "Confidence": 99.62132,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.014053625,
                  "Left": 0.524055,
                  "Top": 0.47866535,
                  "Width": 0.055261176
                },
                "Polygon": [
                  {
                    "X": 0.5245105,
                    "Y": 0.47866535
                  },
                  {
                    "X": 0.57931614,
                    "Y": 0.47892413
                  },
                  {
                    "X": 0.57886016,
                    "Y": 0.492719
                  },
                  {
                    "X": 0.524055,
                    "Y": 0.49245575
                  }
                ]
              },
              "Text": "1.63"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.98426,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015571758,
                  "Left": 0.32197675,
                  "Top": 0.49176782,
                  "Width": 0.06639509
                },
                "Polygon": [
                  {
                    "X": 0.322479,
                    "Y": 0.49176782
                  },
                  {
                    "X": 0.38837186,
                    "Y": 0.49208444
                  },
                  {
                    "X": 0.38786894,
                    "Y": 0.5073396
                  },
                  {
                    "X": 0.32197675,
                    "Y": 0.507017
                  }
                ]
              },
              "Text": "TOTAL"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.987755,
              "Text": "TOTAL"
            },
            "ValueDetection": {
              "Confidence": 99.727585,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015366865,
                  "Left": 0.5115432,
                  "Top": 0.4925996,
                  "Width": 0.06802062
                },
                "Polygon": [
                  {
                    "X": 0.5120398,
                    "Y": 0.4925996
                  },
                  {
                    "X": 0.57956386,
                    "Y": 0.492924
                  },
                  {
                    "X": 0.5790666,
                    "Y": 0.50796646
                  },
                  {
                    "X": 0.5115432,
                    "Y": 0.507636
                  }
                ]
              },
              "Text": "26.65"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 98.172745,
              "Text": "VENDOR_ADDRESS"
            },
            "ValueDetection": {
              "Confidence": 82.233574,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.0736636,
                  "Left": 0.23564203,
                  "Top": 0.13027571,
                  "Width": 0.3646074
                },
                "Polygon": [
                  {
                    "X": 0.23802797,
                    "Y": 0.13027571
                  },
                  {
                    "X": 0.6002494,
                    "Y": 0.13123979
                  },
                  {
                    "X": 0.5978458,
                    "Y": 0.2039393
                  },
                  {
                    "X": 0.23564203,
                    "Y": 0.2028193
                  }
                ]
              },
              "Text": "MICHAELS STORE #9010 (386)\nMICHAELS STORE #9010\n5507 S WILL IAMSON BLVD\nPORT ORGANCE FL 0328\nRewards"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.82914,
              "Text": "VENDOR_NAME"
            },
            "ValueDetection": {
              "Confidence": 99.774445,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015820362,
                  "Left": 0.43053368,
                  "Top": 0.77923393,
                  "Width": 0.10620486
                },
                "Polygon": [
                  {
                    "X": 0.43103263,
                    "Y": 0.77923393
                  },
                  {
                    "X": 0.5367385,
                    "Y": 0.7799217
                  },
                  {
                    "X": 0.53623855,
                    "Y": 0.7950543
                  },
                  {
                    "X": 0.43053368,
                    "Y": 0.79435706
                  }
                ]
              },
              "Text": "Michaels"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.261604,
              "Text": "VENDOR_NAME"
            },
            "ValueDetection": {
              "Confidence": 99.254166,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.017535156,
                  "Left": 0.34749302,
                  "Top": 0.74923056,
                  "Width": 0.18318836
                },
                "Polygon": [
                  {
                    "X": 0.34803215,
                    "Y": 0.74923056
                  },
                  {
                    "X": 0.5306814,
                    "Y": 0.750387
                  },
                  {
                    "X": 0.5301402,
                    "Y": 0.7667657
                  },
                  {
                    "X": 0.34749302,
                    "Y": 0.76559156
                  }
                ]
              },
              "Text": "Baron Brothers"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 84.58116,
              "Text": "VENDOR_NAME"
            },
            "ValueDetection": {
              "Confidence": 84.57034,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.016403547,
                  "Left": 0.09846482,
                  "Top": 0.9092639,
                  "Width": 0.1066431
                },
                "Polygon": [
                  {
                    "X": 0.09897771,
                    "Y": 0.9092639
                  },
                  {
                    "X": 0.20510791,
                    "Y": 0.91003776
                  },
                  {
                    "X": 0.20459391,
                    "Y": 0.92566746
                  },
                  {
                    "X": 0.09846482,
                    "Y": 0.9248838
                  }
                ]
              },
              "Text": "Michaels"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.61307,
              "Text": "VENDOR_NAME"
            },
            "ValueDetection": {
              "Confidence": 99.38257,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.067741856,
                  "Left": 0.27484772,
                  "Top": 0.02388506,
                  "Width": 0.37585786
                },
                "Polygon": [
                  {
                    "X": 0.27704704,
                    "Y": 0.02388506
                  },
                  {
                    "X": 0.6507056,
                    "Y": 0.024643417
                  },
                  {
                    "X": 0.6484895,
                    "Y": 0.09162691
                  },
                  {
                    "X": 0.27484772,
                    "Y": 0.09072038
                  }
                ]
              },
              "Text": "Michaels"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.5656,
              "Text": "VENDOR_PHONE"
            },
            "ValueDetection": {
              "Confidence": 99.30803,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.02516729,
                  "Left": 0.513059,
                  "Top": 0.12500818,
                  "Width": 0.18025605
                },
                "Polygon": [
                  {
                    "X": 0.51387364,
                    "Y": 0.12500818
                  },
                  {
                    "X": 0.693315,
                    "Y": 0.12547939
                  },
                  {
                    "X": 0.69249743,
                    "Y": 0.15017547
                  },
                  {
                    "X": 0.513059,
                    "Y": 0.14967802
                  }
                ]
              },
              "Text": "767-7495\n(386)"
            }
          },
          {
            "PageNumber": 1,
            "Type": {
              "Confidence": 77.8634,
              "Text": "VENDOR_PHONE"
            },
            "ValueDetection": {
              "Confidence": 77.85874,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.017437564,
                  "Left": 0.678982,
                  "Top": 0.6728925,
                  "Width": 0.08273666
                },
                "Polygon": [
                  {
                    "X": 0.6795431,
                    "Y": 0.6728925
                  },
                  {
                    "X": 0.7617187,
                    "Y": 0.67337465
                  },
                  {
                    "X": 0.7611566,
                    "Y": 0.6903301
                  },
                  {
                    "X": 0.678982,
                    "Y": 0.68983966
                  }
                ]
              },
              "Text": "273283"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 40.54405,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.01861322,
                  "Left": 0.3452497,
                  "Top": 0.80828625,
                  "Width": 0.105018266
                },
                "Polygon": [
                  {
                    "X": 0.34583968,
                    "Y": 0.80828625
                  },
                  {
                    "X": 0.45026797,
                    "Y": 0.808984
                  },
                  {
                    "X": 0.44967672,
                    "Y": 0.82689947
                  },
                  {
                    "X": 0.3452497,
                    "Y": 0.8261906
                  }
                ]
              },
              "Text": "Apply at"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 40.557293,
              "Text": "VENDOR_URL"
            },
            "ValueDetection": {
              "Confidence": 39.797928,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.020134445,
                  "Left": 0.46005422,
                  "Top": 0.8064932,
                  "Width": 0.22246507
                },
                "Polygon": [
                  {
                    "X": 0.46066916,
                    "Y": 0.8064932
                  },
                  {
                    "X": 0.6825193,
                    "Y": 0.80797225
                  },
                  {
                    "X": 0.6819016,
                    "Y": 0.8266277
                  },
                  {
                    "X": 0.46005422,
                    "Y": 0.82512414
                  }
                ]
              },
              "Text": "michaels.com/jobs"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 95.59342,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015813794,
                  "Left": 0.20817952,
                  "Top": 0.47752836,
                  "Width": 0.118048854
                },
                "Polygon": [
                  {
                    "X": 0.20868088,
                    "Y": 0.47752836
                  },
                  {
                    "X": 0.32622838,
                    "Y": 0.47808364
                  },
                  {
                    "X": 0.3257258,
                    "Y": 0.49334216
                  },
                  {
                    "X": 0.20817952,
                    "Y": 0.49277627
                  }
                ]
              },
              "Text": "Sales Tax"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 95.7012,
              "Text": "OTHER"
            },
            "ValueDetection": {
              "Confidence": 94.76301,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.0148161845,
                  "Left": 0.3351267,
                  "Top": 0.47790813,
                  "Width": 0.05507071
                },
                "Polygon": [
                  {
                    "X": 0.3356061,
                    "Y": 0.47790813
                  },
                  {
                    "X": 0.3901974,
                    "Y": 0.47816595
                  },
                  {
                    "X": 0.38971743,
                    "Y": 0.49272433
                  },
                  {
                    "X": 0.3351267,
                    "Y": 0.49246183
                  }
                ]
              },
              "Text": "6.5%"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 98.94095,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.016538816,
                  "Left": 0.16235839,
                  "Top": 0.55539703,
                  "Width": 0.23201784
                },
                "Polygon": [
                  {
                    "X": 0.1628617,
                    "Y": 0.55539703
                  },
                  {
                    "X": 0.39437622,
                    "Y": 0.55659795
                  },
                  {
                    "X": 0.39387053,
                    "Y": 0.57193583
                  },
                  {
                    "X": 0.16235839,
                    "Y": 0.5707138
                  }
                ]
              },
              "Text": "Application Label:"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.33988,
              "Text": "OTHER"
            },
            "ValueDetection": {
              "Confidence": 99.28804,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.01631376,
                  "Left": 0.40673432,
                  "Top": 0.55517536,
                  "Width": 0.14436783
                },
                "Polygon": [
                  {
                    "X": 0.40724728,
                    "Y": 0.55517536
                  },
                  {
                    "X": 0.55110216,
                    "Y": 0.5559203
                  },
                  {
                    "X": 0.55058765,
                    "Y": 0.5714891
                  },
                  {
                    "X": 0.40673432,
                    "Y": 0.5707309
                  }
                ]
              },
              "Text": "VISA CREDIT"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.92816,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.014597928,
                  "Left": 0.16142683,
                  "Top": 0.5711607,
                  "Width": 0.052309345
                },
                "Polygon": [
                  {
                    "X": 0.16189739,
                    "Y": 0.5711607
                  },
                  {
                    "X": 0.21373618,
                    "Y": 0.57143444
                  },
                  {
                    "X": 0.21326512,
                    "Y": 0.5857586
                  },
                  {
                    "X": 0.16142683,
                    "Y": 0.58548045
                  }
                ]
              },
              "Text": "AID:"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.97742,
              "Text": "OTHER"
            },
            "ValueDetection": {
              "Confidence": 99.76837,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.01582389,
                  "Left": 0.22786072,
                  "Top": 0.5704441,
                  "Width": 0.18383013
                },
                "Polygon": [
                  {
                    "X": 0.22834887,
                    "Y": 0.5704441
                  },
                  {
                    "X": 0.41169086,
                    "Y": 0.5714112
                  },
                  {
                    "X": 0.41120088,
                    "Y": 0.586268
                  },
                  {
                    "X": 0.22786072,
                    "Y": 0.58528477
                  }
                ]
              },
              "Text": "A0000000031010"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.94725,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.014741978,
                  "Left": 0.16102745,
                  "Top": 0.5860232,
                  "Width": 0.051816773
                },
                "Polygon": [
                  {
                    "X": 0.16150267,
                    "Y": 0.5860232
                  },
                  {
                    "X": 0.21284422,
                    "Y": 0.5862989
                  },
                  {
                    "X": 0.2123685,
                    "Y": 0.60076517
                  },
                  {
                    "X": 0.16102745,
                    "Y": 0.6004851
                  }
                ]
              },
              "Text": "TVR:"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.97238,
              "Text": "OTHER"
            },
            "ValueDetection": {
              "Confidence": 99.959145,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015226345,
                  "Left": 0.22527339,
                  "Top": 0.58647823,
                  "Width": 0.13385978
                },
                "Polygon": [
                  {
                    "X": 0.22575025,
                    "Y": 0.58647823
                  },
                  {
                    "X": 0.35913315,
                    "Y": 0.58719444
                  },
                  {
                    "X": 0.35865498,
                    "Y": 0.60170454
                  },
                  {
                    "X": 0.22527339,
                    "Y": 0.6009768
                  }
                ]
              },
              "Text": "0880008000"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.912056,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.015172367,
                  "Left": 0.1609792,
                  "Top": 0.6024514,
                  "Width": 0.05061236
                },
                "Polygon": [
                  {
                    "X": 0.16146863,
                    "Y": 0.6024514
                  },
                  {
                    "X": 0.21159156,
                    "Y": 0.6027254
                  },
                  {
                    "X": 0.21110162,
                    "Y": 0.61762375
                  },
                  {
                    "X": 0.1609792,
                    "Y": 0.61734533
                  }
                ]
              },
              "Text": "TSI:"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.99429,
              "Text": "OTHER"
            },
            "ValueDetection": {
              "Confidence": 99.96867,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.01478845,
                  "Left": 0.22399686,
                  "Top": 0.60285467,
                  "Width": 0.056731135
                },
                "Polygon": [
                  {
                    "X": 0.22447298,
                    "Y": 0.60285467
                  },
                  {
                    "X": 0.28072798,
                    "Y": 0.6031622
                  },
                  {
                    "X": 0.28025132,
                    "Y": 0.6176431
                  },
                  {
                    "X": 0.22399686,
                    "Y": 0.61733073
                  }
                ]
              },
              "Text": "E800"
            }
          },
          {
            "LabelDetection": {
              "Confidence": 99.72834,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.021488799,
                  "Left": 0.18840627,
                  "Top": 0.63513714,
                  "Width": 0.44219688
                },
                "Polygon": [
                  {
                    "X": 0.18902892,
                    "Y": 0.63513714
                  },
                  {
                    "X": 0.63060313,
                    "Y": 0.6376365
                  },
                  {
                    "X": 0.6299749,
                    "Y": 0.6566259
                  },
                  {
                    "X": 0.18840627,
                    "Y": 0.654077
                  }
                ]
              },
              "Text": "Previous Michaels Rewards Balance:"
            },
            "PageNumber": 1,
            "Type": {
              "Confidence": 99.74252,
              "Text": "OTHER"
            },
            "ValueDetection": {
              "Confidence": 99.70965,
              "Geometry": {
                "BoundingBox": {
                  "Height": 0.014841939,
                  "Left": 0.64129055,
                  "Top": 0.6368945,
                  "Width": 0.055914484
                },
                "Polygon": [
                  {
                    "X": 0.64177114,
                    "Y": 0.6368945
                  },
                  {
                    "X": 0.697205,
                    "Y": 0.637208
                  },
                  {
                    "X": 0.69672394,
                    "Y": 0.65173644
                  },
                  {
                    "X": 0.64129055,
                    "Y": 0.6514182
                  }
                ]
              },
              "Text": "$0.0"
            }
          }
        ]
      }
    ]
  }
  "#;
  let inputData: InputData = serde_json::from_str(json).unwrap();
  let outputData = process(inputData);
  println!("{:?}", outputData);
}

    

