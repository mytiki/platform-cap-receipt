use async_trait::async_trait;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::Client as S3Client;
use lambda_http::tracing;
use serde_json::de::Read;

#[async_trait]
pub trait GetFile {
    async fn get_file(&self, bucket: &str, key: String) -> Result<Vec<u8>, GetObjectError>;
}

#[async_trait]
pub trait GetFileList {
    async fn get_file_list(&self, bucket: &str) -> Result<Vec<String>, GetObjectError>;
}

#[async_trait]
impl GetFile for S3Client {
    async fn get_file(&self, bucket: &str, key: String) -> Result<Vec<u8>, GetObjectError> {
        tracing::info!("get file bucket {}, key {}", bucket, key);

        let output = self.get_object().bucket(bucket).key(key).send().await;


        return match output {
            Ok(response) => {
                let bytes = response.body.collect().await.unwrap().to_vec();
                tracing::info!("Object is downloaded, size is {}", bytes.len());
                Ok(bytes)
            }
            Err(err) => {
                let service_err = err.into_service_error();
                let meta = service_err.meta();
                tracing::info!("Error from aws when downloding: {}", meta.to_string());
                Err(service_err)
            }
        };
    }
}

#[async_trait]
impl GetFileList for S3Client {
    async fn get_file_list(&self, bucket: &str) -> Result<Vec<String>, GetObjectError> {
        let mut json_list: Vec<String> =  Vec::new();
        let mut response = self
            .list_objects_v2()
            .bucket(bucket.to_owned())
            .max_keys(10)
            .into_paginator()
            .send();
        while let Some(result) = response.next().await {
            return match result {
                Ok(output) => {
                    for object in output.contents.unwrap_or_default() {
                        let key = object.key.unwrap();
                        let byte = self.get_file(bucket, key).await?;
                        let json_file:String = serde_json::from_slice(&*byte).unwrap();
                        json_list.push(json_file);
                    }
                    Ok(json_list)
                }
                Err(err) => {
                    let service_err = err.into_service_error();
                    let meta = service_err.meta();
                    tracing::info!("Error from aws when listing: {}", meta.to_string());
                    Ok(json_list)
                }
            }
        }
        Ok(json_list)
    }
}
