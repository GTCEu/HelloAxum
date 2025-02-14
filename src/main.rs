#![allow(unused)]

use std::{net::SocketAddr};
use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::{get, get_service}, Router};
use serde::{ser::Impossible, Deserialize};
use tower_http::{services::ServeDir};

#[tokio::main]
async fn main() {
    let router_all = Router::new()
    .merge(routes_hello())
    .fallback_service(route_static());
    
    let address= SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("!!LITSENING on {address}\n");
    axum_server::bind(address)
        .serve(router_all.into_make_service())
        .await
        .unwrap();

    fn route_static() -> Router {
        Router::new().fallback_service(get_service(ServeDir::new("./")))
    }

    fn routes_hello() -> Router {
        Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
    }

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>
    }

    // e.g. `/hello?name=param`
    async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
        println!("!! {:<12} - handler_hello - {params:?}", "HANDLER");

        let name = params.name.as_deref().unwrap_or("Missing");

        Html(format!(" 🐈 Hello {name} ") )
    }

    // e.g. `/hello2/param`
    async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
        println!("!! {:<12} - handler_hello - {name:?}", "HANDLER");

        Html(format!(" 🐈 Hello {name} ") )
    };

} //$ cargo install cargo-watch
//$ cargo watch -q -c -w src/ -x run