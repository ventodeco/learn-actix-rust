use std::sync::RwLock;

use actix_web::{body::BoxBody, http::{header::ContentType, StatusCode}, web::{get, resource, Data, Path, Redirect}, App, Either, Error, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use serde::Serialize;
use derive_more::{Display, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = Data::new(
        AppState {
            users: RwLock::new(vec![
                User { id: 1, user_name: "vento".to_string(), full_name: "Vento Deco".to_string() },
                User { id: 2, user_name: "deco".to_string(), full_name: "Tes".to_string() },
                User { id: 3, user_name: "uhuy".to_string(), full_name: "Uhuy".to_string() }
            ])
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(
                resource("/user/{user_name}")
                    .route(get().to(get_user))
            )
            .service(
                resource("/na")
                    .route(get().to(failure_msg))
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

struct AppState {
    users: RwLock<Vec<User>>
}

#[derive(Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub full_name: String,
}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let user_result = serde_json::to_string(&self);

        match user_result {
            Ok(user) => {
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(user)
            },
            Err(_) => {
                HttpResponse::InternalServerError()
                    .content_type(ContentType::json())
                    .body("Failed to serialize User")
            }
        }
    }
}

#[allow(unused)]
#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "Internal Error")]
    Internal,
    #[display(fmt = "Unknown Error")]
    Unknown,
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::Unknown => StatusCode::NOT_FOUND,
        }
    }
}

async fn get_user(app_data: Data<AppState>, path: Path<String>)
    -> Either<Result<impl Responder, Error>, Result<User, Error>> {
    let user_name = path.into_inner();

    let users = app_data.users.read().unwrap();
    let user_result = users
        .iter()
        .find(|user| user.user_name == user_name);

    match user_result {
        Some(user) if user.id != 3 => Either::Left(Ok(Redirect::new("/", "../na"))),
        Some(user) => Either::Right(Ok(user.clone())),
        None => Either::Right(Err(MyError::Internal.into()))
    }
}

async fn failure_msg() -> &'static str {
    "unknown error has occurred"
}
