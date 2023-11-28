use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error_route() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("SERVER ERROR").to_string(),
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error_route));

    Ok(router.into())
}
