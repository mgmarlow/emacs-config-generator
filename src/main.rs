use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use eyre::Result;
use features::Features;
use hyper::StatusCode;
use languages::{eglot, Languages};
use query_extractor::Query;
use serde::Deserialize;
use tower_http::services::ServeDir;

mod features;
mod languages;
mod query_extractor;

pub trait ConfigBuilder {
    fn build_string(options: Option<Vec<String>>) -> String;
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .route("/config", get(config))
        .fallback(handler_404);

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct EmacsConfig {
    theme: String,
    font_family: String,
    feature: Option<Vec<String>>,
    language: Option<Vec<String>>,
}

#[derive(Template)]
#[template(path = "init.txt")]
struct ConfigTemplate {
    theme: String,
    font_family: String,
    eglot: String,
    features: String,
    languages: String,
}

impl Into<ConfigTemplate> for EmacsConfig {
    fn into(self) -> ConfigTemplate {
        ConfigTemplate {
            font_family: if self.font_family.len() == 0 {
                String::from("Monaco")
            } else {
                self.font_family
            },
            theme: if self.theme == "light" {
                "'ef-duo-light"
            } else {
                "'ef-autumn"
            }
            .to_string(),
            eglot: eglot(self.language.clone().unwrap_or_default()),
            features: Features::build_string(self.feature),
            languages: Languages::build_string(self.language),
        }
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 not found")
}

async fn config(Query(conf): Query<EmacsConfig>) -> impl IntoResponse {
    let template: ConfigTemplate = conf.into();
    PlainTextTemplate(template)
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

struct PlainTextTemplate<T>(T);

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

impl<T> IntoResponse for PlainTextTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(txt) => txt.into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
