use actix_web::{ App, HttpServer, web::Data };

use dotenv::dotenv;
use std::env;

mod routes;
use routes::*;

mod database;
use database::*;

fn config() -> (String, u16, String) {
    dotenv().ok();

    let domain = env::var("BASE_DOMAIN").unwrap_or("127.0.0.1".to_string());
    let port = env
        ::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap();
    let url_database = env::var("URL_DATABASE").unwrap();

    (domain, port, url_database)
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (domain, port, url_database) = config();

    let database = database_connection(url_database).await.expect("Failed to connect to database");

    println!("Database connection established");

    let server = HttpServer::new(move ||
        App::new()
            .app_data(Data::new(database.clone()))
            .service(home)
            .service(hello_user)
            .service(create_new_user)
            .service(get_all_users)
            .service(create_new_todo)
            .service(get_all_todos)
            .service(update_todo_title)
            .service(update_todo_description)
            .service(mark_todo_completed)
            .service(delete_a_todo)
    )
        .bind((domain.clone(), port))?
        .run();

    println!("Server Running at http:/{}:{}", domain, port);
    server.await
}
