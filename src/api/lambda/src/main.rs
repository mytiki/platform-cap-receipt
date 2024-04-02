async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
  let resp = Response::builder()
      .status(404)
      .header("content-type", "text/plain")
      .body({Body::from("NOT IMPLEMENTED")})
  Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
