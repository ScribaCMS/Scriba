mod index;
mod post;

use super::DbConn;
use rocket::{get, routes};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/post/<id>")]
pub fn post(id: i32, conn: DbConn) -> String {
    use schema::posts::dsl::*;

    let post = posts
        .find(id)
        .first::<Post>(&conn)
        .expect("Error loading post");

    format!("Title: {}\n\n{}", post.title, post.body)
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/", routes![index, post])
}
