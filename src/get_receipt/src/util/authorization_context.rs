/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use lambda_http::request::RequestContext;
use std::error::Error;

pub struct AuthorizationContext {
    id: String,
    namespace: String,
    scopes: Vec<String>,
}

impl AuthorizationContext {
    pub fn new(request_context: &RequestContext) -> Result<Self, Box<dyn Error>> {
        let fields = request_context
            .authorizer()
            .unwrap()
            .clone()
            .fields;
        let namespace = fields
            .get("namespace")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let id = fields.get("id")
          .unwrap()
          .as_str()
          .unwrap()
          .to_string();
        let scopes = fields
            .get("scopes")
            .map_or(None, serde_json::Value::as_str)
            .unwrap()
            .split(" ")
            .map(|scope|{ scope.to_string() } )
            .collect();
        Ok(Self {
            id,
            namespace,
            scopes
        })
    }

    pub fn id(&self) -> &String {
      &self.id
    }
    
    pub fn namespace(&self) -> &String {
      &self.namespace
    }

    pub fn scopes(&self) -> &Vec<String> {
      &self.scopes
    }
}
