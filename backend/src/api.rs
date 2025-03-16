use actix_web::{get, post};

#[get("/api")]
async fn get_api() -> &'static str {
    "Hello from GET /api"
}

#[post("/api")]
async fn post_api() -> &'static str {
    "Hello from POST /api"
}