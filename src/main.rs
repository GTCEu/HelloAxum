#![allow(unused)]

pub use self::error::{Error,Result};

use std::{net::SocketAddr};
use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use model::ModelController;
use serde::{ser::Impossible, Deserialize};
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()>{
    // init mc
    let mc = ModelController::new().await?;
    
    let routes_apis = web::routes_tickets::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let router_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", routes_apis)
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(route_static());
    
    let address= SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("!!LITSENING on {address}\n");
    axum_server::bind(address)
        .serve(router_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("!! {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}    

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
}

//$ cargo install cargo-watch
//$ cargo watch -q -c -w src/ -x run