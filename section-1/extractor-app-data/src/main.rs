use std::sync::Mutex;

use actix_web::{App, HttpServer};


struct Messenger {
    message: String
}

struct MutableState {
    messenger: Mutex<Messenger>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_data = actix_web::web::Data::new(MutableState {
        messenger: Mutex::new(Messenger {
            message: "hello".to_string()
        })
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
                .route("/", actix_web::web::post().to(update))
                .route("/", actix_web::web::get().to(get))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await

}

async fn update(app_data: actix_web::web::Data<MutableState>) -> String {

    let mut messenger = app_data.messenger.lock().unwrap();
    messenger.message = format!("{} world", messenger.message);
    "".to_string()
}

async fn get(app_data: actix_web::web::Data<MutableState>) -> String {
    app_data.messenger.lock().unwrap().message.clone()
}
