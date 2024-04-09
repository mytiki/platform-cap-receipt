use async_trait::async_trait;
use aws_sdk_s3::{operation::{get_object::GetObjectError, list_objects_v2::ListObjectsV2Error}, Client as S3Client};

#[async_trait]
pub trait GetFile {
    async fn get_file(&self, bucket: &str, key: String) -> Result<Vec<u8>, GetObjectError>;
}

#[async_trait]
pub trait GetFileList {
    async fn get_file_list(&self, bucket: &str, path: &str) -> Result<Vec<String>, ListObjectsV2Error>;
}

#[async_trait]
impl GetFile for S3Client {
    async fn get_file(&self, bucket: &str, key: String) -> Result<Vec<u8>, GetObjectError> {
        let output = self.get_object().bucket(bucket).key(key).send().await;
        return match output {
            Ok(response) => {
                let bytes = response.body.collect().await.unwrap().to_vec();
                Ok(bytes)
            }
            Err(err) => {
                let service_err = err.into_service_error();
                let meta = service_err.meta();
                Err(service_err.into())
            }
        };
    }
}

#[async_trait]
impl GetFileList for S3Client {
    async fn get_file_list(&self, bucket: &str, path: &str) -> Result<Vec<String>, ListObjectsV2Error> {
      let mut json_list: Vec<String> = Vec::new();
      let mut response = self
          .list_objects_v2()
          .bucket(bucket.to_owned())
          .prefix(path.to_owned())
          .max_keys(10)
          .into_paginator()
          .send();
      while let Some(result) = response.next().await {
        match result {
          Ok(output) => {
              for object in output.contents.unwrap_or_default() {
                  let key = &object.key.unwrap();
                  let byte = self.get_file(bucket, key.clone()).await.unwrap();
                  let json_file: String = String::from_utf8(byte).unwrap();
                  json_list.push(json_file);
              }
          }
          Err(err) => {
              let service_err = err.into_service_error();
              let meta = service_err.meta();
          }
        }
      }
      Ok(json_list)
    }
}
