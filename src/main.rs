#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket::http::Cookie;
use rocket::http::Cookies;

use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
  title: String,
  parent: &'static str,
}

#[get("/")]
fn index(mut cookies: Cookies) -> Template {
  let cookie = Cookie::build("int", "val").finish();
  let intval = cookies
    .get("int")
    .unwrap_or(&Cookie::build("int", "N-O-N-E").finish())
    .value()
    .to_string();

  cookies.add(cookie);

  Template::render(
    "index",
    &TemplateContext {
      title: intval,
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
