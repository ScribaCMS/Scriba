use super::DbConn;
use rocket::{get, http::Status, request::FromParam, response::status::Custom};
use schema::posts::dsl::*;

#[derive(Debug, FromForm)]
struct PostData {
    title: String,
    body: String,
}

#[get("/post/<id>")]
pub fn post(id: i32, conn: DbConn) -> Result<String, Custom<String>> {
    let post = posts
        .find(id)
        .first::<Post>(&conn)
        .map_err(|e| Custom(Status::NotFound, format!("{:?}", e)))?;
    Ok(format!("Title: {}\n\n{}", post.title, post.body))
}

#[get("/post/new", data = "<post_data>")]
pub fn new_post(post_data: Form<PostData>, conn: DbConn) -> Result<String, String> {
    let post_data = post_data.get();
    if super::handlers::create_post(&conn, &post_data.title, &post_data.body) {
        Ok(format!("Successfully created post: {}", post_data.title))
    } else {
        Err(format!("Error creating post: {}", post_data.title))
    }
}
