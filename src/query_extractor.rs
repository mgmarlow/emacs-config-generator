use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
    Error,
};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use serde_qs::Config;

// A custom extractor that uses serde_qs to support arrays in query strings
// e.g. language[]=go&language[]=lua.
pub struct Query<T>(pub T);

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = QueryRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, QueryRejection> {
        let query = parts.uri.query().unwrap_or_default();
        let config = Config::new(4, false);
        let value = config
            .deserialize_str(query)
            .map_err(|err| QueryRejection(Error::new(err)))?;
        Ok(Query(value))
    }
}

#[derive(Debug)]
pub struct QueryRejection(Error);

impl IntoResponse for QueryRejection {
    fn into_response(self) -> Response {
        match self {
            Self(inner) => (
                StatusCode::BAD_REQUEST,
                format!("Failed to deserialize query string: {}", inner),
            )
                .into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::Request;
    use hyper::Body;
    use serde::Deserialize;
    use std::fmt::Debug;

    use super::*;

    async fn check<T>(uri: impl AsRef<str>, value: T)
    where
        T: DeserializeOwned + PartialEq + Debug,
    {
        let (mut parts, _) = Request::builder()
            .uri(uri.as_ref())
            .body(Body::empty())
            .unwrap()
            .into_parts();

        assert_eq!(
            Query::<T>::from_request_parts(&mut parts, &())
                .await
                .unwrap()
                .0,
            value
        );
    }

    #[tokio::test]
    async fn test_array_params() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Example {
            language: Option<Vec<String>>,
            foo: Option<String>,
        }

        check(
            "https://ecg.com",
            Example {
                language: None,
                foo: None,
            },
        )
        .await;

        check(
            "https://ecg.com?language[]=go&language[]=rust&foo=bar",
            Example {
                language: Some(vec![String::from("go"), String::from("rust")]),
                foo: Some(String::from("bar")),
            },
        )
        .await;
    }
}
