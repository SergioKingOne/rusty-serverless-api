use lambda_runtime::{service_fn, LambdaEvent};
use rusty_serverless_api::{create_function, delete_function, read_function, update_function};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    // Here you can route to different functions based on the event
    // For simplicity, let's assume one function per Lambda
    Ok(())
}
