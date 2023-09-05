use axum::{
    routing::get,
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
        .route("/css", get(css))
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
    
    let rendered = TEMPLATES.render("foo/index.html", &context).unwrap();
    
    Html(rendered)
}

async fn css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    let css = std::fs::read_to_string("dist/output.css").unwrap();
    
    (headers, css)
}
