use axum::{response::{ IntoResponse, Response }, http::StatusCode, body };
use sqlx::error::Error;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum MyError {
    DBError(Error)
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        match self {
            MyError::DBError(ref err) => {
                let body = body::Body::from(err.to_string());
                Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(body).unwrap()
            },
            _ => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(body::Body::default()).unwrap()
        }
    }
}