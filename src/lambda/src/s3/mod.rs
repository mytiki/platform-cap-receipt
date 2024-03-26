
mod s3_client;

use std::env;

use aws_sdk_s3::Client as S3Client;
use lambda_http::lambda_runtime::Error;
use s3_client::{GetFile, GetFileList};

pub async fn get(provider: String, address: String, receipt_id: String) -> Result<Vec<String>, Error> {
  
  let bucket = env::var("TIKI_BUCKET").expect("TIKI_BUCKET is not set");

  let shared_config = aws_config::load_from_env().await;
  let client = S3Client::new(&shared_config);
  let client_ref = &client;

  let list: Vec<String> = client_ref.get_file_list(
    format!("arn:aws:s3:::{bucket}/{provider}/{address}/{receipt_id}").as_str()).await?;
  
  let mut results = vec![];

  for obj in list {
    let file = client_ref.get_file(&bucket, obj.clone()).await?;
    let json_str = String::from_utf8(file)?;
    results.push(json_str);
  }
  
  Ok(results)

}
