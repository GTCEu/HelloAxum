#![allow(unused)]

use std::net::SocketAddr;
use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::{Deserialize};

#[tokio::main]
async fn main() {
    let router_hello: Router = Router::new().route(
        "/hello",
        get(handler_hello));
    
    let address= SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("!!LITSENING on {address}\n");
    axum_server::bind(address)
        .serve(router_hello.into_make_service())
        .await
        .unwrap();
    
    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>
    }

    async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
        println!("!! {:<12} - handler_hello - {params:?}", "HANDLER");

        let name = params.name.as_deref().unwrap_or("Missing");

        Html(format!(" 🐈 Hello {name} "))
    }
} //$ cargo install cargo-watch
//$ cargo watch -q -c -w src/ -x run