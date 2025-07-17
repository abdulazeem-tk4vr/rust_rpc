mod api;
mod config;
mod methods;
mod middleware;
mod utils;
mod shardeum;

use std::sync::Arc;
use api::{RpcRequest, RpcResponse};
use axum::{extract::Json, routing::post, Router, extract::State};
use config::Config;

#[derive(Clone)]
pub struct SharedState {
    /// Instance of the `Liberdus` backend library.
    shardeum: Arc<shardeum::Shardeum>,
}

async fn rpc_method_handler(
    State(state): State<SharedState>,
    Json(payload): Json<RpcRequest>
) -> Json<RpcResponse> {
    let method = payload.method.clone();
    let response = match method.as_str() {
        "dummy" => methods::lib_dummy(payload),
        _ => api::generate_error_response(404, "Incorrect method entered".to_string(), payload.id),
    };
    if state.shardeum.config.verbose {
        println!("The verbose flags are enabled with response {:?}", Json(response.clone()));
    }
    Json(response)
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

    let shm = Arc::new(shardeum::Shardeum{
        config: Arc::new(config.clone())
    });

    let state = SharedState {
        shardeum: shm,
    };

    let app = Router::new().route("/", post(rpc_method_handler)).with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
