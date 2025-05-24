// src/main.rs
mod domain;
mod repository;
mod usecase;
mod services;
mod controller;

use axum::{Router, routing::post};
use axum_server::Server;
use std::sync::Arc;
use crate::services::service_user::InMemoryUserRepository;
use crate::usecase::register_user::RegisterUserUseCase;
use crate::controller::user_handler::{find_user_handler, register_user_handler};

#[tokio::main]
async fn main() {
    let repo = InMemoryUserRepository::new();
    let usecase = Arc::new(RegisterUserUseCase::new(repo));

    let app = Router::new()
        .route("/register_user", post(register_user_handler::<InMemoryUserRepository>))
        .route("/find_user", post(find_user_handler::<InMemoryUserRepository>))
        .with_state(usecase);

    Server::bind("0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
