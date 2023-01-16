use rocket_dyn_templates::{Template, context};

#[get("/signup")]
pub async fn signup() -> Template {
    Template::render("user/auth", context!{
        document_title: "Sign up",
        signup: true
    })
}

/*
    TODO:
    Create a POST getter for '/user/signup' that stores a valid user on mongodb,
    then redirect to '/user/login' to log into the new user account.
*/

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