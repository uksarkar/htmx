use axum::{
    body::Body,
    extract::Request,
    http::{header::ACCEPT, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

use crate::ExtAppState;

#[derive(Debug, Clone)]
pub struct HxResponse {
    data: Value,
    template: String,
}

impl IntoResponse for HxResponse {
    fn into_response(self) -> Response {
        let mut status = StatusCode::NOT_IMPLEMENTED.into_response();
        status.extensions_mut().insert(self);

        status
    }
}

impl HxResponse {
    pub fn new(template: impl Into<String>, data: Value) -> Self {
        Self {
            data,
            template: template.into(),
        }
    }

    pub fn view(template: impl Into<String>) -> Self {
        Self::new(template, json!({}))
    }
}

pub async fn response_middleware(request: Request<Body>, next: Next) -> Response {
    let is_htmx_request = request
        .headers()
        .get("HX-Request")
        .is_some_and(|h| h.to_str().is_ok_and(|v| v == "true"));
    let accept = request
        .headers()
        .get(&ACCEPT)
        .map(|h| h.as_ref().to_owned());
    let state = request
        .extensions()
        .get::<ExtAppState>()
        .unwrap()
        .to_owned();

    let mut response = next.run(request).await;

    if let Some(hx_response) = &response.extensions_mut().remove::<HxResponse>() {
        return match accept.as_deref() {
            Some(b"application/json") => Json(hx_response.data.clone()).into_response(),
            _ => {
                let mut html = state
                    .h
                    .render(&hx_response.template, &hx_response.data)
                    .unwrap(); // ignoring template rendering errors

                if !is_htmx_request && hx_response.template != "index" {
                    html = state.h.render("index", &json!({"content": html})).unwrap();
                }

                Html(html).into_response()
            }
        };
    }

    return response;
}
