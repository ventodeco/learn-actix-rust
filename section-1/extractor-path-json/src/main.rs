use actix_web::{web::{self, Json, Path}, App, HttpServer};
use serde::Deserialize;


#[derive(Deserialize)]
struct EntityId {
    id: i64
}

#[derive(Clone)]
struct FinalUser {
    id: i64,
    user_name: String,
    full_name: String
}

#[derive(Deserialize)]
struct NewUser {
    user_name: String,
    full_name: String
}

struct AppState {
    users: std::sync::RwLock<Vec<FinalUser>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_data = actix_web::web::Data::new(
        AppState {
            users: std::sync::RwLock::new(vec![
                FinalUser { id: 1, user_name: "Vento".to_string(), full_name: "Vento Deco".to_string() },
                FinalUser { id: 2, user_name: "HEHE".to_string(), full_name: "HEHE Deco".to_string() },
                FinalUser { id: 3, user_name: "HUHU".to_string(), full_name: "HUHU Deco".to_string() },
                FinalUser { id: 4, user_name: "LAKSD".to_string(), full_name: "LAKSD Deco".to_string() }
            ])
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(
                web::scope("/v1")
                    .service(
                        web::resource("user/{id}")
                            .route(web::get().to(get_user_name))
                    )
                    .service(
                        web::resource("/user")
                            .route(web::post().to(insert_user))
                        )
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

async fn insert_user(app_data: web::Data<AppState>, new_user: Json<NewUser>) -> String {
    let mut users = app_data.users.write().unwrap();
    let max_id = users.iter().max_by_key(|usr| { usr.id }).unwrap().id;

    print!("max_id: {}", max_id);

    users.push(FinalUser { id: max_id + 1, user_name: new_user.user_name.clone(), full_name: new_user.full_name.clone()});

    users.last().unwrap().user_name.to_string()
}

async fn get_user_name(app_data: web::Data<AppState>, params: Path<EntityId>) -> String {
    let users = app_data.users.read().unwrap();

    users
        .iter()
        .find(
            |user| {
                user.id == params.id
            }
        )
        .unwrap()
        .clone()
        .user_name
}
