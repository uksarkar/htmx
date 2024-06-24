use axum::{extract::{Path, Query}, response::Html, routing::get, Extension, Router};
use serde_json::json;

use crate::{pagination::{Pagination, QueryPagination}, response::HxResponse, ExtAppState};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/users", get(users))
        .route("/users/:id/view", get(view_user))
        .route("/about", get(about))
        .fallback(notfound)
}

async fn index(Extension(state): Extension<ExtAppState>) -> Html<String> {
    let html = state.h.render("index", &json!({})).unwrap();
    
    Html(html)
}

async fn users(Extension(state): Extension<ExtAppState>, Query(query_pagination): Query<QueryPagination>) -> HxResponse {
    let pagination = Pagination::new(state.db.len(), query_pagination.page);

    let users = &state.db[pagination.from..pagination.to];
    
    HxResponse::new("user.list", json!({"users": users, "pagination": pagination}))
}

async fn view_user(Extension(state): Extension<ExtAppState>, Path(id): Path<u32>) -> HxResponse {
    let user = &state.db.iter().find(|u| u.id == id).unwrap();

    HxResponse::new("user.view", json!(user))
}

async fn about() -> HxResponse {
    HxResponse::view("about")
}

async fn notfound() -> HxResponse {
    HxResponse::view("404")
}