use mongodb::bson::{doc, Document};
use rocket_dyn_templates::{
    Template, 
    context
};
use rocket::{
    form::Form, 
    response::{Redirect, Flash}, 
    http::{CookieJar, Cookie, SameSite},
    request::FlashMessage
};
use rocket_db_pools::Connection;

// Basic sign up page, only need a template with a form for user input.
#[get("/signup")]
pub async fn signup(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("user/auth", context!{
        document_title: "Sign up",
        signup: true,
        error: flash.map(|flash| format!("{}: {}", flash.kind(), flash.message())).unwrap_or_else(|| "".to_string())
    })
}

// Take form data and compair to our database before creating account.
#[derive(FromForm)]
pub struct AuthForm<'r> {
    username: &'r str,
    password: &'r str
}

#[post("/signup", data="<auth>")]
pub async fn acc_creation(db: Connection<crate::UsersDB>, auth: Form<AuthForm<'_>>) -> Result<Redirect, Flash<Redirect>> {
    // Let's prevent a useless database call. (you can absolutely do this client side with JS.)
    if auth.username.is_empty() | auth.password.is_empty() {
        return Err(Flash::error(Redirect::to(uri!("/user", signup)), "You can't leave the form empty!"));
    }

    let collection = db.database("auth").collection::<Document>("users");
    let search = collection.find_one(doc!{"username": auth.username}, None).await.unwrap();

    match search {
        Some(_) => Err(Flash::error(Redirect::to(uri!("/user", signup)), "User already exists. Try logging in if you own the account.")),
        None => { // If account doesn't exist, create one.
            let creator = collection.insert_one(doc!{ "username": auth.username, "password": auth.password}, None).await;
            match creator {
                Ok(_) => Ok(Redirect::to(uri!("/user", login))),
                Err(error) => Err(Flash::error(Redirect::to(uri!("/user", signup)), error.to_string()))
            }
        }
    }
}

// Provide basic template for user input
#[get("/login")]
pub async fn login(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("user/auth", context!{
        document_title: "Sign up",
        signup: false,
        error: flash.map(|flash| format!("{}: {}", flash.kind(), flash.message())).unwrap_or_else(|| "".to_string())
    })
}

// Compair form data to our database and create private cookie if we own the existing account.
/*
    BUG:
     
*/
#[post("/login", data = "<auth>")]
pub async fn access_acc(jar: &CookieJar<'_>, db: Connection<crate::UsersDB>, auth: Form<AuthForm<'_>>) -> Result<Redirect, Flash<Redirect>>{
    // Let's prevent a useless database call.
    if auth.username.is_empty() | auth.password.is_empty() {
        return Err(Flash::error(Redirect::to(uri!("/user", login)), "You can't leave the form empty!"));
    }
    
    let collection = db.database("auth").collection::<Document>("users");
    let search = collection.find_one(doc!{"username": auth.username}, None).await.unwrap();

    match search {
        Some(user) => {
            // Check password.
            let pass = user.get_str("password").expect("Missing password!");
            if auth.password != pass {
                return Err(Flash::error(Redirect::to(uri!("/user", login)), "Incorrect password."));
            }

            // Create private cookie with authorized username.
            let account = user.get_str("username").expect("Missing username");
            let priv_cookie = Cookie::build("authorizedUser", account.to_owned())
                .path("/")
                .secure(true)
                .same_site(SameSite::Strict);
            jar.add_private(priv_cookie.finish());

            Ok(Redirect::to(uri!("/user", user_account)))
        },
        None => Err(Flash::error(Redirect::to(uri!("/user", login)), "User does not exist."))
    }
}

// Render account/user data from database using private cookie as proof we own the account.
#[get("/account")]
pub async fn user_account(jar: &CookieJar<'_>, db: Connection<crate::UsersDB>) -> Result<Template, Flash<Redirect>> {
    let auth_user = jar.get_private("authorizedUser");

    match auth_user {
        None => Err(Flash::error(Redirect::to(uri!("/user", login)), "You aren't signed in!")),
        Some(user) => {
            let current_user = db.database("auth")
                                                             .collection::<Document>("users")
                                                             .find_one(doc!{"username": user.value()}, None)
                                                             .await
                                                             .unwrap();
            match current_user {
                Some(account) => Ok(
                    Template::render("user/account", context! {
                        document_title: "Account Dashbord",
                        username: &account.get("username"),
                        password: &account.get("password")
                    })
                ),
                None => Err(Flash::error(Redirect::to(uri!("/user", login)), "Cannot find account."))
            }
        }
    }
}