use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;

use crate::usecase::register_user::RegisterUserUseCase;
use crate::repository::user_repository::UserRepository;

#[derive(Deserialize)]
pub struct RegisterUserInput {
    pub id: String,
    pub name: String,
}

pub async fn register_user_handler<R: UserRepository>(
    State(usecase): State<Arc<RegisterUserUseCase<R>>>,
    Json(input): Json<RegisterUserInput>,
) {
    // let usecase = RegisterUserUseCase::new();
    usecase.execute(input.id, input.name).await
}

pub async fn find_user_handler<R: UserRepository>(
    State(usecase): State<Arc<RegisterUserUseCase<R>>>,
    Json(input): Json<RegisterUserInput>,
) -> impl IntoResponse {
    match usecase.find(input.id).await {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => (StatusCode::NOT_FOUND, "User not found").into_response(),
    }
}
