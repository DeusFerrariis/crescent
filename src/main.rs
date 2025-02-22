use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    let state = AppState::new().unwrap();
    // TODO: live reload of config file

    let app = Router::new()
        .route("/api/{*wildcard}", get(route_handler))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone, Debug)]
struct AppState {
    config: Config,
}

impl AppState {
    fn new() -> Result<AppState, ()> {
        Ok(AppState {
            config: load_config().unwrap(),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Config {
    pub routes: Vec<Route>,
}

#[derive(Clone, Debug, Deserialize)]
struct Route {
    pub path: String,
    pub endpoint_url: String,
    // pub partial_path: Option<bool>,
}

impl Route {
    async fn forward_request(&self) -> (HeaderMap, String) {
        let Ok(resp) = reqwest::get(self.endpoint_url.as_str()).await else {
            return (HeaderMap::new(), "fail".to_string());
        };

        let headers = resp.headers().clone();
        let body = resp.text().await.unwrap();

        return (headers, body);
    }
}

async fn route_handler(
    State(state): State<AppState>,
    Path(path_tail): Path<String>,
) -> impl IntoResponse {
    match state
        .config
        .routes
        .iter()
        .find(|r| r.path == "/".to_string() + path_tail.as_str())
    {
        Some(route) => route.forward_request().await,
        None => (HeaderMap::new(), "not_found".to_string()),
    }
}

fn load_config() -> Result<Config, ()> {
    let config_string = std::fs::read_to_string("./config.toml").unwrap();
    let config: Config = toml::from_str(&config_string).unwrap();
    return Ok(config);
}
