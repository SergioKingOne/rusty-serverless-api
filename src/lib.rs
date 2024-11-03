pub mod create;
pub mod delete;
pub mod error;
pub mod read;
pub mod update;

use async_trait::async_trait;
use lambda_runtime::{Context, Error};
use serde_json::Value;

#[async_trait]
pub trait Handler {
    async fn handle(&self, event: Value, context: Context) -> Result<Value, Error>;
}
