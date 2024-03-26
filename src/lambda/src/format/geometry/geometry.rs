/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Geometry {
    bounding_box: BoundingBox,
    polygon: Vec<Polygon>,
}
