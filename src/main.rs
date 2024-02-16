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
async fn poke_weight(Path(pokedex): Path<u32>) -> String {
  let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex);
  let res =
    reqwest::get(url).await.expect("poke-api failed").text().await.expect("could not get text");
  let res: PokeWeight = from_str(&res).expect("could not deserialize");
  info!("number: {pokedex}, weight: {}", res.weight);
  (res.weight as f32 / 10f32).to_string()
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
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }));

  Ok(router.into())
}
