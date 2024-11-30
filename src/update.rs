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

    let key = HashMap::from([(
        "id".to_string(),
        AttributeValue::S(event.payload.id.clone()),
    )]);

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":d".to_string(),
        AttributeValue::S(event.payload.data.clone()),
    );

    let mut expression_attribute_names = HashMap::new();
    expression_attribute_names.insert("#d".to_string(), "data".to_string());

    client
        .update_item()
        .table_name("rusty-serverless-dynamodb-table")
        .set_key(Some(key))
        .update_expression("SET #d = :d")
        .set_expression_attribute_values(Some(expression_attribute_values))
        .set_expression_attribute_names(Some(expression_attribute_names))
        .send()
        .await
        .map_err(DynamoError::from)?;

    Ok(UpdateResponse {
        message: format!("Item with id {} updated.", event.payload.id),
    })
}
