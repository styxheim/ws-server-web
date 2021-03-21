use actix_web::{get, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_derive::Serialize;
use std::path::Path;
mod race_struct;

const DB_PATH: &str = "./db";

#[derive(Serialize)]
struct RaceContext {
  title: String,
}

#[get("/race/{race_id}")]
async fn race(
  hb: web::Data<Handlebars<'_>>,
  web::Path(race_id): web::Path<u64>,
) -> HttpResponse {
  let body = hb
    .render(
      "race",
      &RaceContext {
        title: format!("Race id {}", race_id).into(),
      },
    )
    .unwrap();

  HttpResponse::Ok().body(body)
}

#[derive(Serialize)]
struct IndexContext {
  race_list: Vec<race_struct::Race>,
  title: String,
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
  let body = hb
    .render(
      "index",
      &IndexContext {
        race_list: race_struct::list_races(&Path::new(DB_PATH)),
        title: "0".to_string(),
      },
    )
    .unwrap();

  HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let bind_string = "0.0.0.0:80";
  println!("Listen on {}", bind_string);

  let mut handlebars = Handlebars::new();
  handlebars
    .register_templates_directory(".hbs", "./templates")
    .unwrap();
  let handlebars_ref = web::Data::new(handlebars);

  HttpServer::new(move || {
    App::new()
      .app_data(handlebars_ref.clone())
      .service(index)
      .service(race)
  })
  .bind(bind_string)?
  .run()
  .await
}
