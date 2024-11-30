use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::{Context, Error as LambdaError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ReadRequest {
    pub id: String,
}

#[derive(Serialize)]
pub struct ReadResponse {
    id: String,
    data: String,
}

pub async fn handler(event: ReadRequest, _: Context) -> Result<ReadResponse, LambdaError> {
    // Extract the `id` from query string parameters
    let id = event.id;

    // Set up the DynamoDB client
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let client = Client::new(&config);

    // Create the DynamoDB key
    let key = HashMap::from([("id".to_string(), AttributeValue::S(id.clone()))]);

    // Query the DynamoDB table
    let resp = client
        .get_item()
        .table_name("rusty-serverless-dynamodb-table")
        .set_key(Some(key))
        .send()
        .await
        .map_err(DynamoError::from)?;

    // Parse the response
    if let Some(item) = resp.item {
        let data = item
            .get("data")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| LambdaError::from("Missing 'data' field in the item"))?
            .to_string();
        Ok(ReadResponse { id, data })
    } else {
        Err(LambdaError::from(format!("Item with id {} not found.", id)))
    }
}
