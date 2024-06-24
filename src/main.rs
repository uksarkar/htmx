use std::sync::Arc;

use axum::{middleware::from_fn, Extension, Router};
use db::User;
use fake::{Fake, Faker};
use response::response_middleware;
use routes::routes;

mod db;
mod response;
mod routes;
mod pagination;

pub struct AppState {
    h: handlebars::Handlebars<'static>,
    db: Vec<User>,
}

pub type ExtAppState = Arc<AppState>;

#[tokio::main]
async fn main() {
    let mut h = handlebars::Handlebars::new();

    // register templates
    h.register_template_file("index", "templates/index.hbs")
        .expect("Unable to register template");
    h.register_template_file("user.list", "templates/user/list.hbs")
        .expect("Unable to register template");
    h.register_template_file("user.view", "templates/user/view.hbs")
        .expect("Unable to register template");
    h.register_template_file("404", "templates/404.hbs")
        .expect("Unable to register template");
    h.register_template_file("about", "templates/about.hbs")
        .expect("Unable to register template");

    // initialize db
    let db: Vec<User> = (0..100).map(|_| Faker.fake::<User>()).collect();

    let state = Arc::new(AppState { h, db });

    let app = Router::from(routes())
        .layer(from_fn(response_middleware))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening to: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
