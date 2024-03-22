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
    page: i64,
    text_type: TextType,
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

// Function to process blocks
fn process_blocks(blocks: &Vec<Block>) -> Vec<StateResponseBlock> {
    blocks.iter().map(|block| {
        StateResponseBlock{
            confidence: block.confidence,
            text: block.text,
        }
    }).collect()
} 

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_process_blocks() {

        let json_data =  r#"
        {
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
        }
        "#;


        process_blocks(json_data);

    }}

