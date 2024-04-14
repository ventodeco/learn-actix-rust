use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;


#[derive(Serialize)]
pub struct Person {
  name: String
}

impl Responder for Person {
  type Body = BoxBody;

  fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
    let result = serde_json::to_string(&self);

    match result {
      Ok(item) => HttpResponse::Ok().content_type(ContentType::json()).body(item),
      Err(_) => HttpResponse::InternalServerError().content_type(ContentType::plaintext()).body("Failure!")
    }
  }
}

pub async fn get_user_name() -> Person {
  Person {
    name: "Vento".to_string()
  }
}
