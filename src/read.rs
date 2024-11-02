use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct ReadRequest {
    id: String,
}

#[derive(Serialize)]
struct ReadResponse {
    id: String,
    data: String,
}

pub async fn handler(event: ReadRequest, _: Context) -> Result<ReadResponse, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let key = HashMap::from([("id".to_string(), AttributeValue::S(event.id.clone()))]);

    let resp = client
        .get_item()
        .table_name("RustyServerlessAPI")
        .set_key(Some(key))
        .send()
        .await
        .map_err(DynamoError::from)?;

    if let Some(item) = resp.item {
        let id = item
            .get("id")
            .and_then(|v| v.as_s().ok())
            .unwrap_or_default()
            .to_string();
        let data = item
            .get("data")
            .and_then(|v| v.as_s().ok())
            .unwrap_or_default()
            .to_string();
        Ok(ReadResponse { id, data })
    } else {
        Err("Item not found".into())
    }
}

pub fn read_function() -> impl Fn(ReadRequest, Context) -> _ {
    handler_fn(handler)
}
