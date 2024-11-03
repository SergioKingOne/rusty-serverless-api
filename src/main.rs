use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

use rusty_serverless_api::{
    create::{handler as create_handler, CreateRequest},
    delete::{handler as delete_handler, DeleteRequest},
    read::{handler as read_handler, ReadRequest},
    update::{handler as update_handler, UpdateRequest},
};

async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // Extract HTTP method from the event
    let http_method = event
        .payload
        .get("httpMethod")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::from("Missing httpMethod"))?;

    // Extract the body if it exists
    let body = event
        .payload
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or("{}");

    match http_method {
        "POST" => {
            let request: CreateRequest = serde_json::from_str(body)?;
            let response = create_handler(request, event.context).await?;
            Ok(json!(response))
        }
        "GET" => {
            let request: ReadRequest = serde_json::from_str(body)?;
            let response = read_handler(request, event.context).await?;
            Ok(json!(response))
        }
        "PUT" => {
            let request: UpdateRequest = serde_json::from_str(body)?;
            let lambda_event = LambdaEvent::new(request, event.context);
            let response = update_handler(lambda_event).await?;
            Ok(json!(response))
        }
        "DELETE" => {
            let request: DeleteRequest = serde_json::from_str(body)?;
            let response = delete_handler(request, event.context).await?;
            Ok(json!(response))
        }
        _ => Err(Error::from(format!(
            "Unsupported HTTP method: {}",
            http_method
        ))),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
