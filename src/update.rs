use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct UpdateRequest {
    id: String,
    data: String,
}

#[derive(Serialize)]
pub struct UpdateResponse {
    message: String,
}

pub async fn handler(event: LambdaEvent<UpdateRequest>) -> Result<UpdateResponse, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let client = Client::new(&config);

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":data".to_string(),
        AttributeValue::S(event.payload.data.clone()),
    );

    client
        .update_item()
        .table_name("rusty-serverless-dynamodb-table")
        .key("id", AttributeValue::S(event.payload.id.clone()))
        .update_expression("SET data = :data")
        .set_expression_attribute_values(Some(expression_attribute_values))
        .send()
        .await
        .map_err(DynamoError::from)?;

    Ok(UpdateResponse {
        message: format!("Item with id {} updated.", event.payload.id),
    })
}
