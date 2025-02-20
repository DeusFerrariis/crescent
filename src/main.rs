use axum::{extract::Path, routing::get, Router};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    pub routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    pub path: String,
    pub endpoint_url: String,
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let app = Router::new().route("/api/{*wildcard}", get(route_handler));

    axum::serve(listener, app).await.unwrap();
}

async fn route_handler(Path(path_tail): Path<String>) -> String {
    let cfg = std::fs::read_to_string("./config.toml").unwrap();

    let config: Config = toml::from_str(&cfg).unwrap();
    println!("Routes: {:?}", config.routes);

    for r in config.routes.iter() {
        if r.path == "/".to_string() + &path_tail {
            return r.endpoint_url.clone();
        }
    }

    "not found".to_string()
}
