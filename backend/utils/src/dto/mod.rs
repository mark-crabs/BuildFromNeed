use actix_web::error;
use derive_more::derive::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationDb {
    pub offset: i64,
    pub limit: i64,
}

impl Pagination {
    pub fn limit_and_offset(&self) -> PaginationDb {
        let offset =
            (self.page.unwrap_or(1).saturating_sub(1) * self.page_size.unwrap_or(10)) as i64;
        let limit = self.page_size.unwrap_or(10) as i64;

        PaginationDb { offset, limit }
    }
}

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
