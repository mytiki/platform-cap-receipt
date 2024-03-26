mod s3;
mod format;
mod util;

use util::authorization_context::AuthorizationContext;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
  tracing::init_default_subscriber();
  let context = AuthorizationContext::new(&event.request_context()).unwrap();
  let provider = context.provider().unwrap();
  let address = context.address().unwrap();
  let receipt_id = event.path_parameters().first("receipt_id").unwrap().to_owned();
  let results = s3::get(provider, address, receipt_id).await;  
  let formatted = results.iter().map(|result: String| { 
    format::process(result) 
  });

  let resp = Response::builder()
      .status(200)
      .header("content-type", "application/json")
      .body(formatted.into())
      .map_err(Box::new)?;
  Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
