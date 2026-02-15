#[macro_use]
extern crate rocket;

use std::env;

use diesel::{Connection, PgConnection, prelude::*};
use rocket::{fairing::AdHoc, http::Header, serde::json::Json};
use serde::Serialize;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = todos_api::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[get("/todos")]
fn list_todos() -> Json<Vec<Todo>> {
    use todos_api::schema::todos::dsl::*;
    let connection = &mut connect_db();
    let results = todos
        .select(Todo::as_select())
        .load(connection)
        .expect("failed to query todos");
    Json(results)
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
