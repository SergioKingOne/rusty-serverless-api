use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct CreateRequest {
    id: String,
    data: String,
}

#[derive(Serialize)]
struct CreateResponse {
    message: String,
}

pub async fn handler(event: CreateRequest, _: Context) -> Result<CreateResponse, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S(event.id.clone()));
    item.insert("data".to_string(), AttributeValue::S(event.data));

    client
        .put_item()
        .table_name("RustyServerlessAPI")
        .set_item(Some(item))
        .send()
        .await
        .map_err(DynamoError::from)?;

    Ok(CreateResponse {
        message: format!("Item with id {} created.", event.id),
    })
}

pub fn create_function() -> impl Fn(CreateRequest, Context) -> _ {
    handler_fn(handler)
}
