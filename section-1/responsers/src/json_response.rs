use actix_web::{web::Json, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Person {
  name: String
}

pub async fn get_user_name() -> impl Responder {
  Json(Person {
      name: "Vento".to_string()
  })
}
