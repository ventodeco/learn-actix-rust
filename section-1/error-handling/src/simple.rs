pub struct MyError {
  pub msg: &'static str,
  pub detail: String
}

pub async fn get() -> actix_web::Result<String> {
  let err = Err(MyError { msg: "unknown internal error", detail: "our not shown error".to_string() });

  err.map_err(|err| actix_web::error::ErrorInternalServerError(err.msg))
}
