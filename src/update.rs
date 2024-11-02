use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct UpdateRequest {
    id: String,
    data: String,
}

#[derive(Serialize)]
struct UpdateResponse {
    message: String,
}

pub async fn handler(event: UpdateRequest, _: Context) -> Result<UpdateResponse, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(":data".to_string(), AttributeValue::S(event.data.clone()));

    client
        .update_item()
        .table_name("RustyServerlessAPI")
        .key("id", AttributeValue::S(event.id.clone()))
        .update_expression("SET data = :data")
        .set_expression_attribute_values(Some(expression_attribute_values))
        .send()
        .await
        .map_err(DynamoError::from)?;

    Ok(UpdateResponse {
        message: format!("Item with id {} updated.", event.id),
    })
}

pub fn update_function() -> impl Fn(UpdateRequest, Context) -> _ {
    handler_fn(handler)
}
