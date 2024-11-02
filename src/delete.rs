use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct DeleteRequest {
    id: String,
}

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn handler(event: DeleteRequest, _: Context) -> Result<DeleteResponse, Error> {
    let config = aws_config::load_from_env().await;
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

pub fn delete_function() -> impl Fn(DeleteRequest, Context) -> _ {
    handler_fn(handler)
}
