
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
