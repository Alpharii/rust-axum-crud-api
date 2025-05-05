use axum::{Router, routing::{get, post, delete}};
use crate::handlers::*;
use deadpool_postgres::Pool;

pub fn create_routes(pool: Pool) -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", delete(delete_user))
        .with_state(pool)
}
