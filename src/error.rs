use aws_sdk_dynamodb::Error as DynamoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Dynamo(DynamoError),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl From<DynamoError> for Error {
    fn from(err: DynamoError) -> Self {
        Error::Dynamo(err)
    }
}
