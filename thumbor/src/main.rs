mod pb;
use pb::*;

use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::convert::TryInto;

use axum::{extract::Path, http::StatusCode, routing::get, Router};

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

async fn generator(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url {} \n spe: {:#?}", url, spec))
}

#[tokio::main]
async fn main() {
    println!("init subscriber");
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new().route("/image/:spec/:url", get(generator));

    // run it with hyper on localhost:3000
    tracing::debug!("listening on localhost:8090");
    axum::Server::bind(&"0.0.0.0:8090".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
