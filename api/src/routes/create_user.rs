use actix_web::{ get, post, web::{ Data, Json }, HttpResponse, Responder };
use serde::{ Serialize, Deserialize };
use sqlx::MySqlPool;

use crate::{ routes::{ logging, User }, TypeDbError };

#[derive(Serialize, Deserialize)]
struct CreateUser {
    first_name: String,
    last_name: String,
}

#[post("/user/create")]
pub async fn create_new_user(db: Data<MySqlPool>, body: Json<CreateUser>) -> impl Responder {
    logging("POST: /user/create");

    let response = sqlx
        ::query(
            "
            INSERT INTO users(first_name, last_name) 
            VALUES( ?, ?)
        "
        )
        .bind(&body.first_name)
        .bind(&body.last_name)
        .execute(&**db).await;

    match response {
        Ok(id) => {
            HttpResponse::Created().json(User {
                id: id.last_insert_id() as i32,
                first_name: body.first_name.clone(),
                last_name: body.last_name.clone(),
            })
        }
        Err(_e) =>
            HttpResponse::InternalServerError().json(TypeDbError {
                error: _e.to_string(),
            }),
    }
}

#[get("/users/all")]
pub async fn get_all_users(db: Data<MySqlPool>) -> impl Responder {
    logging("GET: /users/get_all");
    let res: Result<Vec<User>, sqlx::Error> = sqlx
        ::query_as("
            SELECT id, first_name, last_name
            FROM users
        ")
        .fetch_all(&**db).await;

    match res {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_e) =>
            HttpResponse::InternalServerError().json(TypeDbError {
                error: _e.to_string(),
            }),
    }
}
