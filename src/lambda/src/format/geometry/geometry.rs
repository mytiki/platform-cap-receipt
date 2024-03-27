/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use super::{bouding_box::BoundingBox, polygon::Polygon};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Geometry {
    bounding_box: BoundingBox,
    polygon: Vec<Polygon>,
}
