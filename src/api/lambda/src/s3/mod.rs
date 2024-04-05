
mod s3_client;

use std::env;

use aws_sdk_s3::Client as S3Client;
use lambda_http::lambda_runtime::Error;
use s3_client::GetFileList;

pub async fn get(path: &String, receipt_id: &String) -> Result<Vec<String>, Error> {
  
  let bucket = env::var("TIKI_BUCKET").unwrap_or("mytiki-us-east-2-publish-data-receipt".to_string());
  let shared_config = aws_config::load_from_env().await;
  let client = S3Client::new(&shared_config);
  let client_ref = &client;

  let results: Vec<String> = client_ref.get_file_list(
    &bucket, format!("result/{}/{}", path, receipt_id).as_str()).await?;
  
  Ok(results)

}
