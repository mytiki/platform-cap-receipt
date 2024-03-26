
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::operation::get_object::GetObjectError;
use lambda_runtime::{Error, tracing};
use serde::Deserialize;
use S3::{GetFile, GetFileList, PutFile, ReceiptData};

mod S3;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let shared_config = aws_config::load_from_env().await;
    let client = S3Client::new(&shared_config);
    let client_ref = &client;

    let mut list: Result<Vec<ReceiptData>, GetObjectError> = client_ref.get_file_list("arn:aws:s3:::test-bucket-rust-tiki/result/").await;
    let mut json_array: String = "".to_owned();
    let unwrapped_list= list.unwrap();
    for receipt in unwrapped_list {
        let mut json = json_array.clone();
        json.push_str(", ");
        json.push_str(&*receipt.to_json());
        json_array = json;
    }

    json_array.to_string().remove(0);
    json_array = (&*(json_array.to_string() + "]}")).parse().unwrap();
    json_array = (&*("{ receipts: [".to_owned() + &*json_array)).parse().unwrap();
    println!("{json_array}");
    Ok(())
}
