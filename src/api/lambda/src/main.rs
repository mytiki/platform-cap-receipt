mod format;
mod s3;
mod util;

use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use serde_json::json;
use util::{authorization_context::AuthorizationContext, input_data::InputData};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let context = AuthorizationContext::new(&event.request_context()).unwrap();
    let path = context.id();
    let receipt_id = event
        .path_parameters()
        .first("receiptId")
        .unwrap()
        .to_owned();
    let results = s3::get(path, &receipt_id).await.unwrap();
    let formatted: Vec<String> = results
        .iter()
        .map(|result| {
            let input: InputData = serde_json::from_str(result).unwrap();
            let json = format::process(input).unwrap();
            println!("{}", json);
            json
        })
        .collect();
    let result = json!(formatted).to_string();
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(result))
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use lambda_http::{aws_lambda_events::query_map::QueryMap, http::StatusCode};

    #[tokio::test]
    async fn test_function_handler() {
        let request = Request::new(
          Body::Binary(vec![]))
          .with_path_parameters({
            let mut hashMap: HashMap<String,String> = HashMap::new();
            hashMap.insert("receipt_id".to_string(),"test".to_string());
            QueryMap::from(hashMap)
          }
          )
          .with_lambda_context(
            serde_json::from_str(r#"
            {
              "accountId": "123456789012",
              "resourceId": "123456",
              "stage": "prod",
              "request_id": "c6af9ac6-7b61-11e6-9a41-93e8deadbeef",
              "requestTime": "09/Apr/2015:12:34:56 +0000",
              "requestTimeEpoch": 1428582896000,
              "deadline": 123456,
              "invoked_function_arn": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
              "env_config": {
                "function_name": "my-function",
                "memory": 123,
                "version": "",
                "log_stream": "",
                "log_group": ""
              },
              "identity": {
                "sourceIp": "127.0.0.1",
                "userAgent": "Custom User Agent String",
                "identityId": "",
                "identityPoolId": ""
              },
              "requestId": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
              "client_context": {
                "id": "provider-id:address",
                "namespace": "addr",
                "scopes": "publish"
              },
              "path": "/prod/path/to/resource",
              "resourcePath": "/{proxy+}",
              "httpMethod": "POST",
              "apiId": "1234567890",
              "protocol": "HTTP/1.1"
            }
            "#).unwrap(),
          )
          .with_request_context(
            serde_json::from_str(r#"
            {
              "accountId": "123456789012",
              "resourceId": "123456",
              "stage": "prod",
              "requestId": "c6af9ac6-7b61-11e6-9a41-93e8deadbeef",
              "requestTime": "09/Apr/2015:12:34:56 +0000",
              "requestTimeEpoch": 1428582896000,
              "authorizer" : {
                "id": "provider-id:address",
                "namespace": "addr",
                "scopes": "publish"
              },
              "identity": {
                "cognitoIdentityPoolId": null,
                "accountId": null,
                "cognitoIdentityId": null,
                "caller": null,
                "accessKey": null,
                "sourceIp": "127.0.0.1",
                "cognitoAuthenticationType": null,
                "cognitoAuthenticationProvider": null,
                "userArn": null,
                "userAgent": "Custom User Agent String",
                "user": null
              },
              "path": "/prod/path/to/resource",
              "resourcePath": "/{proxy+}",
              "httpMethod": "POST",
              "apiId": "1234567890",
              "protocol": "HTTP/1.1"
            }"#).unwrap()
          );
        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
