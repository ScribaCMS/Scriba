use rocket::{ignite, routes};
use scriba::{routes::index, routes::post, database::DbConn};

fn main() {
    ignite()
        .manage(DbConn::fairing())
        .mount("/", routes![index, post])
        .launch();
}
