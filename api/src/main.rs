#[macro_use]
extern crate rocket;

use rocket::{fairing::AdHoc, http::Header, serde::json::Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Todo {
    text: String,
    completed: bool,
}

#[get("/todos")]
fn list_todos() -> Json<Vec<Todo>> {
    let todos = vec![
        Todo {
            text: String::from("implement api"),
            completed: true,
        },
        Todo {
            text: String::from("connect client to api"),
            completed: false,
        },
        Todo {
            text: String::from("connect api to db"),
            completed: false,
        },
    ];
    Json(todos)
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
