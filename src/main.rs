use rocket::fs::{
    FileServer,
    relative
};
use rocket_dyn_templates::Template;
use rocket_db_pools::{
    mongodb,
    Database
};

#[macro_use] extern crate rocket;

mod public; // All publicly accessable pages and API calls.

// Our database
#[derive(Database)]
#[database("mongodb")]
pub struct UsersDB(mongodb::Client);

#[launch]
fn rocket() -> _ {
    rocket::build()

        // Database fairings
        .attach(UsersDB::init())

        // Server rendering
        .attach(Template::fairing())
        .mount("/public", FileServer::from(relative!("public")))

        // Routes
        .mount("/", routes![public::general::index])
        .mount("/user", routes![
            public::user::signup,
            public::user::acc_creation, 
            public::user::login,
            public::user::access_acc,
            public::user::user_account
            ]
        )
}