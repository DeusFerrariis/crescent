use axum::{extract::Path, http::HeaderMap, response::IntoResponse, routing::get, Router};
use reqwest::header;
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

async fn route_handler(Path(path_tail): Path<String>) -> impl IntoResponse {
    let cfg = std::fs::read_to_string("./config.toml").unwrap();

    let config: Config = toml::from_str(&cfg).unwrap();
    println!("Routes: {:?}", config.routes);

    for r in config.routes.iter() {
        let path_parts: Vec<&str> = path_tail.split("/").collect();
        let Some(root) = path_parts.get(0) else {
            break;
        };
        if r.path == "/".to_string() + root {
            let resp = reqwest::get(r.endpoint_url.as_str()).await;
            let mut headers = HeaderMap::new();
            let Ok(body) = resp else {
                return (HeaderMap::new(), "fail".to_string());
            };
            let resp_headers = body.headers().clone();
            for (k, v) in resp_headers {
                let Some(hk) = k.clone() else {
                    continue;
                };
                let header_key = hk.clone();
                let Ok(header_value) = v.to_str() else {
                    continue;
                };
                headers.insert(header_key, header_value.parse().unwrap());
            }
            if let Ok(text) = body.text().await {
                return (headers.to_owned(), text);
            }
            return (HeaderMap::new(), r.endpoint_url.clone());
        }
    }

    return (HeaderMap::new(), "not found".to_string());
}

// NOTE: replace list of routes w/ b tree?
