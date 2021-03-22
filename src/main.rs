use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_derive::Serialize;
use std::path::Path;
mod race_struct;
use rand::Rng;

const DB_PATH: &str = "./db";

#[derive(Serialize)]
struct UserInfo {
  is_authorized: bool,
  username: String,
}

#[derive(Serialize)]
struct RaceContext {
  userinfo: UserInfo,
  title: String,
}

fn user_from_identity(identity: Identity) -> UserInfo {
  return UserInfo {
    is_authorized: identity.identity().is_some(),
    username: identity.identity().unwrap_or("<unknown>".to_owned()),
  };
}

#[get("/race/{race_id}")]
async fn race(
  identity: Identity,
  hb: web::Data<Handlebars<'_>>,
  web::Path(race_id): web::Path<u64>,
) -> HttpResponse {
  let body = hb
    .render(
      "race",
      &RaceContext {
        userinfo: user_from_identity(identity),
        title: format!("Race id {}", race_id).into(),
      },
    )
    .unwrap();

  HttpResponse::Ok().body(body)
}

#[derive(Serialize)]
struct IndexContext {
  userinfo: UserInfo,
  race_list: Vec<race_struct::Race>,
  title: String,
}

#[get("/")]
async fn index(
  identity: Identity,
  hb: web::Data<Handlebars<'_>>,
) -> HttpResponse {
  let body = hb
    .render(
      "index",
      &IndexContext {
        userinfo: user_from_identity(identity),
        race_list: race_struct::list_races(&Path::new(DB_PATH)),
        title: "0".to_string(),
      },
    )
    .unwrap();

  HttpResponse::Ok().body(body)
}

fn get_return_path(req: HttpRequest) -> String {
  match req.headers().get("referer") {
    Some(referer) => referer.to_str().unwrap().to_owned(),
    None => "/".to_owned(),
  }
}

#[post("/login")]
async fn login(req: HttpRequest, identity: Identity) -> HttpResponse {
  identity.remember("unauthorized user".to_owned());
  HttpResponse::Found()
    .header("location", get_return_path(req))
    .finish()
}

#[post("/logout")]
async fn logout(req: HttpRequest, identity: Identity) -> HttpResponse {
  identity.forget();
  HttpResponse::Found()
    .header("location", get_return_path(req))
    .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let bind_string = "0.0.0.0:80";
  println!("Listen on {}", bind_string);

  let private_key = rand::thread_rng().gen::<[u8; 32]>();
  let mut handlebars = Handlebars::new();
  handlebars
    .register_templates_directory(".hbs", "./templates")
    .unwrap();
  let handlebars_ref = web::Data::new(handlebars);

  HttpServer::new(move || {
    App::new()
      .wrap(IdentityService::new(
        CookieIdentityPolicy::new(&private_key)
          .name("auth")
          .secure(true),
      ))
      .app_data(handlebars_ref.clone())
      .service(index)
      .service(race)
      .service(login)
      .service(logout)
  })
  .bind(bind_string)?
  .run()
  .await
}
