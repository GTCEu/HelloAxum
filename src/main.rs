#![allow(unused)]

use std::net::SocketAddr;
use axum::{response::{Html, IntoResponse}, routing::get, Router};

#[tokio::main]
async fn main() {
    let router_hello: Router = Router::new().route(
        "/hello",
        get(handler_func));
    
    let address= SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("!!LITSENING on {address}\n");
    axum_server::bind(address)
        .serve(router_hello.into_make_service())
        .await
        .unwrap();

    async fn handler_func() -> impl IntoResponse {
        println!("!! {:<12} - handler_hello", "HANDLER");

        Html({" 🐈 "})
    }
} //$ cargo install cargo-watch
//$ cargo watch -q -c -w src/ -x run