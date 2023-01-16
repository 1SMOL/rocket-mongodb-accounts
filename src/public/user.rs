use mongodb::bson::{doc, Document};
use rocket_dyn_templates::{
    Template, 
    context
};
use rocket::{
    form::Form, 
    response::{Redirect, Flash}
};
use rocket_db_pools::Connection;

use crate::UsersDB;

#[get("/signup")]
pub async fn signup() -> Template {
    Template::render("user/auth", context!{
        document_title: "Sign up",
        signup: true
    })
}

#[derive(FromForm)]
pub struct AuthForm<'r> {
    username: &'r str,
    password: &'r str
}

#[post("/signup", data="<auth>")]
pub async fn acc_creation(db: Connection<UsersDB>, auth: Form<AuthForm<'_>>) -> Result<Redirect, Flash<Redirect>> {
    let collection = db.database("auth").collection::<Document>("users");
    let search = collection.find_one(doc!{"username": auth.username}, None).await.unwrap();

    match search {
        None => { // If account doesn't exist, create one.
            let creator = collection.insert_one(doc!{ "username": auth.username, "password": auth.password}, None).await;
            match creator {
                Ok(_) => Ok(Redirect::to(uri!(crate::public::user::login))),
                Err(error) => Err(Flash::error(Redirect::to(uri!(signup)), error.to_string()))
            }
        },
        Some(_) => Err(Flash::error(Redirect::to(uri!(signup)), "User already exists."))
    }
}

#[get("/login")]
pub async fn login() -> Template {
    Template::render("user/auth", context!{
        document_title: "Sign up",
        signup: false
    })
}

/*
    TODO:
    Create a POST getter for '/user/login' that checks user creds from mongodb,
    then redirect to a user page with a private cookie that allows the user
    access to their user data. For testing reasons, this data will be stored on mongodb,
    but as dummy data.
*/