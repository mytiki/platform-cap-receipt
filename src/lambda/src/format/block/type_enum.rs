/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "CHILD")]
    Child,
}