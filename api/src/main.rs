#[macro_use]
extern crate rocket;

use std::env;

use diesel::{Connection, PgConnection};
use rocket::{fairing::AdHoc, http::Header, serde::json::Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Todo {
    text: String,
    completed: bool,
}

#[get("/todos")]
fn list_todos() -> Json<Vec<Todo>> {
    let _ = connect_db();
    log::info!("db connected!");
    let todos = vec![
        // Todo {
        //     text: String::from("implement api"),
        //     completed: true,
        // },
        // Todo {
        //     text: String::from("connect client to api"),
        //     completed: true,
        // },
        // Todo {
        //     text: String::from("connect api to db"),
        //     completed: false,
        // },
    ];
    Json(todos)
}

fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_response("cors", |_request, response| {
            Box::pin(async move {
                response.set_header(Header::new(
                    "Access-Control-Allow-Origin",
                    "https://todos-client-wke8.onrender.com",
                ));
            })
        }))
        .mount("/", routes![list_todos])
}
