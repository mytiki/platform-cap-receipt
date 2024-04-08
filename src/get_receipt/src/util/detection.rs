/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

use crate::format::geometry::geometry::Geometry;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Detection {
    pub confidence: Option<f64>,
    pub geometry: Geometry,
    pub text: Option<String>,
}
