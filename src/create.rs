use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::Error;

#[derive(Deserialize)]
pub struct CreateRequest {
    id: String,
    data: String,
}

#[derive(Serialize)]
pub struct CreateResponse {
    message: String,
}

pub async fn handler(event: CreateRequest, _: Context) -> Result<CreateResponse, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let client = Client::new(&config);

    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S(event.id.clone()));
    item.insert("data".to_string(), AttributeValue::S(event.data));

    client
        .put_item()
        .table_name("rusty-serverless-dynamodb-table")
        .set_item(Some(item))
        .send()
        .await
        .map_err(DynamoError::from)?;

    Ok(CreateResponse {
        message: format!("Item with id {} created.", event.id),
    })
}
