/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "CHILD")]
    Child,
}