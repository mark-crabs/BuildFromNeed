use actix_web::error;
use derive_more::derive::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, Serialize, Deserialize)]
#[display("Error: {message}")]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: T,
}

impl<T> DataResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl error::ResponseError for ErrorResponse {}
