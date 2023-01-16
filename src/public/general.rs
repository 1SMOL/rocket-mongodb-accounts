use rocket_dyn_templates::{Template, context};

#[get("/")]
pub async fn index() -> Template {
    Template::render("general/index", context!{
        document_title: "Rocket x MongoDB user auth template"
    })
}