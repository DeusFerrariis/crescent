use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    path: String,
    endpoint_url: String,
}

fn main() {
    println!("Hello, world!");
}
