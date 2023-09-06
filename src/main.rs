use axum::{
    routing::{get, post},
    Router,
    response::{Html, IntoResponse, Result, Response}, http::{HeaderMap, header, StatusCode}
};

use tower_livereload::LiveReloadLayer;
use std::net::SocketAddr;

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate tera;

use tera::{Context, Tera};


#[tokio::main]
async fn main() -> Result<(), AppError> {
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
        .await?;

    Ok(())
}

async fn css() -> Result<impl IntoResponse, AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse()?);
    let css = std::fs::read_to_string("dist/output.css")?;
    
    Ok((headers, css))
}

async fn js() -> Result<impl IntoResponse, AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/javascript".parse()?);
    headers.insert(header::CACHE_CONTROL, "public, max-age=3600".parse()?);
    let js = std::fs::read_to_string("dist/output.js")?;
    
    Ok((headers, js))
}

async fn root_route() -> Result<Html<String>, AppError> {
    let context = Context::new();
    let rendered = render_with_global_context("root/index.html", &context)?;
    
    Ok(Html(rendered))
}

async fn other_route_route() -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("foo", &"hello from the other route");
    let rendered = render_with_global_context("other-route/index.html", &context)?;
    
    Ok(Html(rendered))
}

async fn clicked_uix() -> Result<Html<String>, AppError> {
    Ok(Html("<p class=\"text-center mt-10\">Hello from htmx</p>".to_string()))
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

#[derive(Debug)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}


#[cfg(test)]
mod tests {
    use crate::{root_route, other_route_route, css, js, clicked_uix};

    #[tokio::test]
    async fn css_test() {
        let res = css().await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn js_test() {
        let res = js().await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn root_route_test() {
        let res = root_route().await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn other_route_route_test() {
        let res = other_route_route().await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn clicked_uix_test() {
        let res = clicked_uix().await;
        assert!(res.is_ok());
    }
}
