use lambda_runtime::{service_fn, LambdaEvent};
use rusty_serverless_api::{create, delete, read, update};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    // Initialize the AWS SDK logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // Get the Lambda function name from environment variable
    let function_name =
        std::env::var("AWS_LAMBDA_FUNCTION_NAME").expect("AWS_LAMBDA_FUNCTION_NAME must be set");

    // Route to the appropriate handler based on function name
    let handler = match function_name.as_str() {
        "create-item" => create::handler,
        "read-item" => read::handler,
        "update-item" => update::handler,
        "delete-item" => delete::handler,
        _ => panic!("Unknown function name: {}", function_name),
    };

    lambda_runtime::run(handler).await?;
    Ok(())
}
