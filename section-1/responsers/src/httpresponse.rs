use actix_web::{http::header::ContentType, HttpResponse};

pub async fn get_user_name() -> HttpResponse {
  HttpResponse::Ok()
  .content_type(ContentType::json())
  .insert_header(("test", "test"))
  .body("jim")
}
