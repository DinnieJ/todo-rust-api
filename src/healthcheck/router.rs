use axum::Router;

async fn healthcheck() -> &'static str {
    "OK"
}


pub fn router() -> axum::Router {
    Router::new().route("/", axum::routing::get(healthcheck))
}