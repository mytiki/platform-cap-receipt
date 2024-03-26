/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
pub enum TextType {
    #[serde(rename = "PRINTED")]
    Printed,
}