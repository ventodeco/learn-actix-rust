use actix_web::{body::BoxBody, http::header::ContentType, Error, HttpResponse, ResponseError};
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub enum MyError {
  #[display(fmt = "Internal Server Error")]
  InternalServerError,
  #[display(fmt = "A field value in invalid: {}", field)]
  ValidationError{ field: String },
  #[display(fmt = "An unknown error has occurred")]
  UnknownError
}

impl ResponseError for MyError {
  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code())
    .content_type(ContentType::plaintext())
    .body(self.to_string())
  }

  fn status_code(&self) -> actix_web::http::StatusCode {
    match *self {
        MyError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        MyError::ValidationError { .. } => actix_web::http::StatusCode::BAD_REQUEST,
        MyError::UnknownError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

pub async fn get() -> Result<String, Error> {
  Err(MyError::ValidationError { field: "full_name".to_string() }.into())
}
