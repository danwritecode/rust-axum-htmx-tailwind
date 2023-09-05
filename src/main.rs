use axum::{
    routing::{get, post},
    Router,
    response::{Html, IntoResponse}, http::{HeaderMap, header}
};

use tower_livereload::LiveReloadLayer;
use std::net::SocketAddr;

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate tera;

use tera::{Context, Tera};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/search-results", get(search_results))
        .route("/css", get(css))
        .route("/clicked", post(clicked))
        .layer(LiveReloadLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        Tera::new("templates/**/*").unwrap()
    };
}

async fn root() -> Html<String> {
    let mut context = Context::new();
    context.insert("name", &"world");
    context.insert("foos", &["foo", "bar"]);
    
    let rendered = TEMPLATES.render("root/index.html", &context).unwrap();
    
    Html(rendered)
}

async fn search_results() -> Html<String> {
    let context = Context::new();
    let rendered = TEMPLATES.render("search-results/index.html", &context).unwrap();
    
    Html(rendered)
}

async fn css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    let css = std::fs::read_to_string("dist/output.css").unwrap();
    
    (headers, css)
}

async fn clicked() -> Html<String> {
    Html("It clicked xD".to_string())
}
