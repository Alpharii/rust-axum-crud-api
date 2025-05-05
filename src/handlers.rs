use axum::{Json, extract::{Path, State}};
use uuid::Uuid;
use crate::{models::{User, NewUser}, db};
use deadpool_postgres::Pool;
use serde_json::json;

pub async fn create_user(
    State(pool): State<Pool>,
    Json(payload): Json<NewUser>,
) -> Json<serde_json::Value> {
    let client = pool.get().await.unwrap();
    let id = Uuid::new_v4();

    client.execute(
        "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
        &[&id, &payload.name, &payload.email],
    ).await.unwrap();

    Json(json!({ "status": "created", "id": id }))
}

pub async fn get_users(State(pool): State<Pool>) -> Json<Vec<User>> {
    let client = pool.get().await.unwrap();
    let rows = client.query("SELECT id, name, email FROM users", &[]).await.unwrap();

    let users = rows.iter().map(|row| User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    }).collect();

    Json(users)
}

pub async fn delete_user(State(pool): State<Pool>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    let client = pool.get().await.unwrap();
    client.execute("DELETE FROM users WHERE id = $1", &[&id]).await.unwrap();
    Json(json!({ "status": "deleted" }))
}
