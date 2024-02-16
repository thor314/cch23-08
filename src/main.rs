#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use error::MyError;
use serde::Deserialize;
use serde_json::from_str;
use tracing::info;

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[derive(Deserialize, Debug, Clone)]
struct PokeWeight {
  weight: u32,
}

/// add a GET endpoint /8/weight/<pokedex_number> that, given a pokédex number, responds with the
/// corresponding Pokémon's weight in kilograms as a number in plain text.
///
/// example
/// ```sh
/// curl http://localhost:8000/8/weight/25
///
/// 6
async fn poke_weight(Path(pokedex): Path<u32>) -> String { get_weight(pokedex).await.to_string() }

async fn get_weight(pokedex: u32) -> f64 {
  let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex);
  let res =
    reqwest::get(url).await.expect("poke-api failed").text().await.expect("could not get text");
  let res: PokeWeight = from_str(&res).expect("could not deserialize");
  info!("number: {pokedex}, weight: {}", res.weight);
  res.weight as f64 / 10f64
}
/// Calculate the momentum it would have at the time of impact with the floor if dropped from a 10-meter high chimney.
/// The GET endpoint /8/drop/<pokedex_number> shall respond with a plain text floating point number.
///
/// Use gravitational acceleration g = 9.825 m/s². Ignore air resistance.
async fn poke_drop(Path(pokedex): Path<u32>) -> String {
  let weight = get_weight(pokedex).await;
  let g = 9.825f64;
  // (d=10) = (s=0) * t + 0.5 * g * t^2
  let time: f64 = (2.0 * 10.0 / g).sqrt();
  let speed = g * time;
  let momentum = speed * weight;
  momentum.to_string()
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();

  info!("hello thor");

  let router = Router::new()
    .route("/", get(hello_world))
    .route("/8/weight/:pokedex", get(poke_weight))
    .route("/8/drop/:pokedex", get(poke_drop))
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }));

  Ok(router.into())
}
