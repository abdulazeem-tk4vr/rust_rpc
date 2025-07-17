mod api;
mod config;
mod methods;
mod middleware;
mod utils;

use api::{RpcRequest, RpcResponse};
use axum::{extract::Json, routing::post, Router};
use config::Config;

async fn rpc_method_handler(Json(payload): Json<RpcRequest>) -> Json<RpcResponse> {
    // input => variable : type
    // ...

    let method = payload.method.clone();
    let response = match method.as_str() {
        "dummy" => methods::lib_dummy(payload),
        _ => api::generate_error_response(404, "Incorrect method entered".to_string(), payload.id),
    };
    return Json(response);
}

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap_or_else(|err| {
        eprintln!("Failed to load config: {}", err);
        Config::default()
    });

    println!("The RPC configs are here as follows:");
    println!("Launching the Shardeum RPC server... ");
    println!("Listening on {}:{}", config.host, config.port);
    println!("Node URLs: {:?}", config.node_urls);
    println!("Request timeout: {} seconds", config.request_timeout);
    println!("Verbose mode: {}", config.verbose);

    let app = Router::new().route("/", post(rpc_method_handler));

    // run it
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
