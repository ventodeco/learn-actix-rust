use actix_web::{App, HttpServer};


#[derive(Clone)]
struct Messenger {
    message: String
}

#[derive(Clone)]
struct AppState {
    messenger: Messenger
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = AppState {
        messenger: Messenger { message: "hello".to_string() }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(app_state.clone()))
                .route("/", actix_web::web::post().to(update))
                .route("/", actix_web::web::get().to(get))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

async fn update(app_data: actix_web::web::Data<AppState>) -> String {

    let messenger = app_data.messenger.clone();
    format!("{} world", messenger.message)
}

async fn get(app_data: actix_web::web::Data<AppState>) -> String {
    app_data.messenger.message.clone()
}
