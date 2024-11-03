use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::Context;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Deserialize)]
pub struct DeleteRequest {
    id: String,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    message: String,
}

pub async fn handler(event: DeleteRequest, _: Context) -> Result<DeleteResponse, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let client = Client::new(&config);

    client
        .delete_item()
        .table_name("RustyServerlessAPI")
        .key("id", AttributeValue::S(event.id.clone()))
        .send()
        .await
        .map_err(DynamoError::from)?;

    Ok(DeleteResponse {
        message: format!("Item with id {} deleted.", event.id),
    })
}

// pub fn delete_function() -> impl Fn(DeleteRequest, Context) -> _ {
//     handler_fn(handler)
// }
