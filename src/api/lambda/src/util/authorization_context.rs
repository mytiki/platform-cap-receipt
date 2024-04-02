/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use lambda_http::request::RequestContext;
use std::error::Error;

pub struct AuthorizationContext {
    namespace: Option<String>,
    provider: Option<String>,
    address: Option<String>,
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
            .map_or(None, serde_json::Value::as_str);
        let id = fields.get("id").map_or(None, serde_json::Value::as_str);
        let scopes = fields
            .get("scopes")
            .map_or(None, serde_json::Value::as_str)
            .unwrap()
            .split(" ")
            .map(|scope|{ scope.to_string() } )
            .collect();
        let (provider, address) = match id {
            Some(id) => {
                let split = id.split(":").collect::<Vec<&str>>();
                let provider = split.get(0).map_or(None, |v| Some(v.to_string()));
                let id = split.get(1).map_or(None, |v| Some(v.to_string()));
                (provider, id)
            }
            None => (None, None),
        };
        Ok(Self {
            namespace: namespace.map(str::to_string),
            provider,
            address,
            scopes
        })
    }

    pub fn namespace(&self) -> &Option<String> {
    &self.namespace
    }

    pub fn provider(&self) -> &Option<String> {
      &self.provider
    }

    pub fn address(&self) -> &Option<String> {
      &self.address
    }

    pub fn scopes(&self) -> &Vec<String> {
      &self.scopes
    }
}
