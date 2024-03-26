
/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoundingBox {
    height: f64,
    left: f64,
    top: f64,
    width: f64,
}
