use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use eyre::Result;
use hyper::StatusCode;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/config", get(config));

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Deserialize, Debug, Template)]
#[template(path = "config.html")]
struct ConfigTemplate {
    theme: String,
}

async fn config(Form(conf): Form<ConfigTemplate>) -> impl IntoResponse {
    let template = ConfigTemplate { theme: conf.theme };
    HtmlTemplate(template)
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

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
