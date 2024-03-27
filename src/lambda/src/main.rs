mod s3;
mod format;
mod util;

use serde_json::json;
use util::{authorization_context::AuthorizationContext, input_data::InputData};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
  tracing::init_default_subscriber();
  let context = AuthorizationContext::new(&event.request_context()).unwrap();
  let provider = context.provider().as_ref().clone().unwrap();
  let address = context.address().as_ref().clone().unwrap();
  let receipt_id = event.path_parameters().first("receipt_id").unwrap().to_owned();
  let results = s3::get(provider, address, &receipt_id).await.unwrap();  
  let formatted: Vec<String> = results.iter().map(|result| { 
    let input: InputData = serde_json::from_str(result).unwrap();
    let json = format::process(input).unwrap();
    json
  }).collect();
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
