use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::Error;

#[derive(Deserialize)]
pub struct ReadRequest {
    id: String,
}

#[derive(Serialize)]
pub struct ReadResponse {
    id: String,
    data: String,
}

pub async fn handler(event: ReadRequest, _: Context) -> Result<ReadResponse, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let client = Client::new(&config);

    let key = HashMap::from([("id".to_string(), AttributeValue::S(event.id.clone()))]);

    let resp = client
        .get_item()
        .table_name("rusty-serverless-dynamodb-table")
        .set_key(Some(key))
        .send()
        .await
        .map_err(DynamoError::from)?;

    if let Some(item) = resp.item {
        let id = item
            .get("id")
            .and_then(|v| v.as_s().ok())
            .ok_or(Error::NotFound(format!(
                "Item with id {} not found.",
                event.id
            )))?
            .to_string();
        let data = item
            .get("data")
            .and_then(|v| v.as_s().ok())
            .ok_or(Error::NotFound(format!(
                "Item with id {} not found.",
                event.id
            )))?
            .to_string();
        Ok(ReadResponse { id, data })
    } else {
        Err(Error::NotFound(format!(
            "Item with id {} not found.",
            event.id
        )))
    }
}
