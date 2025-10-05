use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, EntityTrait, Set,
};
use serde::{Deserialize, Serialize};

use crate::{entities::user, AppState};

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<user::Model> for UserResponse {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

pub async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "ok".to_string(),
    })
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let now = chrono::Utc::now().naive_utc();

    let user = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let user = user
        .insert(&state.db)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") || e.to_string().contains("unique constraint") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = user::Entity::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

    Ok(Json(users))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = user::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user.into()))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = user::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut user: user::ActiveModel = user.into();

    if let Some(name) = payload.name {
        user.name = Set(name);
    }

    if let Some(email) = payload.email {
        user.email = Set(email);
    }

    user.updated_at = Set(chrono::Utc::now().naive_utc());

    let user = user
        .update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user.into()))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let user = user::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let user: user::ActiveModel = user.into();

    user.delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

