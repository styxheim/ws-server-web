#![feature(decl_macro)]
use rocket::http::{Cookie, Cookies};
use rocket::{get, routes};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;
use std::path::Path;
mod race_struct;

const DB_PATH: &str = "./db";

#[derive(Serialize)]
struct IndexContext {
  race_list: Vec<race_struct::Race>,
  title: String,
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
    &IndexContext {
      race_list: race_struct::list_races(&Path::new(DB_PATH)),
      title: intval,
    },
  )
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}
