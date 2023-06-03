use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use eyre::Result;
use hyper::StatusCode;
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .route("/config", get(config));

    println!("Listening on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct EmacsConfig {
    theme: String,
    font_family: String,
    helpful: Option<String>,
    vim: Option<String>,
    denote: Option<String>,
    go: Option<String>,
    lua: Option<String>,
    markdown: Option<String>,
    php: Option<String>,
    tsx: Option<String>,
    rust: Option<String>,
    yaml: Option<String>,
    magit: Option<String>,
}

#[derive(Template)]
#[template(path = "init.txt")]
struct ConfigTemplate {
    theme: String,
    font_family: String,
    helpful: bool,
    vim: bool,
    denote: bool,
    go: bool,
    lua: bool,
    markdown: bool,
    php: bool,
    tsx: bool,
    rust: bool,
    yaml: bool,
    magit: bool,
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
            helpful: self.helpful.is_some(),
            vim: self.vim.is_some(),
            denote: self.denote.is_some(),
            go: self.go.is_some(),
            lua: self.lua.is_some(),
            markdown: self.markdown.is_some(),
            php: self.php.is_some(),
            tsx: self.tsx.is_some(),
            rust: self.rust.is_some(),
            yaml: self.yaml.is_some(),
            magit: self.magit.is_some(),
        }
    }
}

async fn config(Form(conf): Form<EmacsConfig>) -> impl IntoResponse {
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
