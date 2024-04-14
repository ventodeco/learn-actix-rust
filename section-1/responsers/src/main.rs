mod impl_response;

use actix_web::{web, App, HttpServer};
use impl_response::get_user_name;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/profile")
                .route(web::get().to(get_user_name))
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
