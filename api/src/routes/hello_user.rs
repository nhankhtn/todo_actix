use actix_web::{ get, http::StatusCode, web::{ Json, Path }, Responder };
use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

use crate::routes::logging;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    fn new(id: i32, first_name: String, last_name: String) -> Self {
        User {
            id,
            first_name,
            last_name,
        }
    }
}

#[get("/hello/:id/{firstname}/{lastname}")]
pub async fn hello_user(params: Path<(u32, String, String)>) -> impl Responder {
    let route = format!("GET: /hello/{}/{}", params.0.clone(), params.1.clone());
    logging(&route);

    let response = User::new(params.0 as i32, params.1.clone(), params.2.clone());
    (Json(response), StatusCode::OK)
}
