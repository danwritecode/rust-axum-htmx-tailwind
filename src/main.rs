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
        .route("/", get(root_route))
        .route("/other-route", get(other_route_route))
        .route("/css", get(css))
        .route("/js", get(js))
        .route("/route-name/uix/clicked", post(clicked_uix))
        .layer(LiveReloadLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    let css = std::fs::read_to_string("dist/output.css").unwrap();
    
    (headers, css)
}

async fn js() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/javascript".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "public, max-age=3600".parse().unwrap());
    let js = std::fs::read_to_string("dist/output.js").unwrap();
    
    (headers, js)
}

async fn root_route() -> Html<String> {
    let context = Context::new();
    let rendered = render_with_global_context("root/index.html", &context).unwrap();
    
    Html(rendered)
}

async fn other_route_route() -> Html<String> {
    let mut context = Context::new();
    context.insert("foo", &"hello from the other route");
    let rendered = render_with_global_context("other-route/index.html", &context).unwrap();
    
    Html(rendered)
}

async fn clicked_uix() -> Html<String> {
    Html("<p class=\"text-center mt-10\">Hello from htmx</p>".to_string())
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new("ui/templates/**/*").unwrap();

        return tera;
    };
}

fn render_with_global_context(template: &str, specific_context: &Context) -> tera::Result<String> {
    let version = env!("CARGO_PKG_VERSION");
    let mut context = Context::new();
    context.insert("cargo_version", &version);

    context.extend(specific_context.clone());
    TEMPLATES.render(template, &context)
}
