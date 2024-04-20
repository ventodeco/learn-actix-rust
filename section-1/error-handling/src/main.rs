mod complex;

use crate::complex::get;
use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", actix_web::web::get().to(get))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
