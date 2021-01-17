#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Hello",
            parent: "layout",
        },
    )
}

fn main() {
    rocket::ignite() 
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
