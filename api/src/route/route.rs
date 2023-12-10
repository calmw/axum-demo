use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, MethodRouter};
use crate::controller::user_controller;
use crate::controller::book_controller;

pub fn app() -> Router {
    Router::new()
        .merge(root())
        .merge(get_user_by_id())
        .merge(get_book_by_id())
        .merge(get_foo()).fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here, 404.")
}

fn root() -> Router {
    async fn handler() -> &'static str {
        "Hello, World!"
    }

    route("/", get(handler))
}

fn get_foo() -> Router {
    async fn handler() -> &'static str {
        "Hi from `GET /foo`"
    }

    route("/foo", get(handler))
}

fn get_user_by_id() -> Router {
    route("/user", get(user_controller::user_controller::get_user_by_id))
}

fn get_book_by_id() -> Router {
    route("/book", get(book_controller::book_controller::get_book_by_id))
}

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

