use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket;

mod public; // All publicly accessable pages and API calls.

#[launch]
fn rocket() -> _ {
    rocket::build()
    
        // Server rendering
        .attach(Template::fairing())
        .mount("/public", FileServer::from(relative!("public")))

        // Routes
        .mount("/", routes![public::general::index])
        .mount("/user", routes![public::user::signup, public::user::login])
}