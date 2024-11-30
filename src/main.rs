use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rusty_serverless_api::{
    create::{handler as create_handler, CreateRequest},
    delete::{handler as delete_handler, DeleteRequest},
    read::{handler as read_handler, ReadRequest},
    update::{handler as update_handler, UpdateRequest},
};
use serde_json::{json, Value};

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

    let response = match http_method {
        "POST" => {
            let request: CreateRequest = serde_json::from_str(body)?;
            let response = create_handler(request, event.context).await?;
            json!(response)
        }
        "GET" => {
            // Extract query parameters for GET
            let query_params = event
                .payload
                .get("queryStringParameters")
                .and_then(Value::as_object)
                .ok_or_else(|| Error::from("Missing queryStringParameters"))?;

            let id = query_params
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::from("Missing 'id' query parameter"))?;

            // Create ReadRequest from query parameters
            let request = ReadRequest { id: id.to_string() };

            // Call the read_handler
            let response = read_handler(request, event.context).await?;
            json!(response)
        }
        "PUT" => {
            let request: UpdateRequest = serde_json::from_str(body)?;
            let lambda_event = LambdaEvent::new(request, event.context);
            let response = update_handler(lambda_event).await?;
            json!(response)
        }
        "DELETE" => {
            let request: DeleteRequest = serde_json::from_str(body)?;
            let response = delete_handler(request, event.context).await?;
            json!(response)
        }
        _ => {
            return Err(Error::from(format!(
                "Unsupported HTTP method: {}",
                http_method
            )))
        }
    };

    Ok(json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": serde_json::to_string(&response)?
    }))
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
