// src/main.rs
mod domain;
mod repository;
mod usecase;
mod services;
mod controller;

use axum::{Router, routing::post};
use axum_server::Server;
use rand::{Rng};
use reqwest::Client;
use std::sync::Arc;
use tokio::{spawn, task::JoinSet, time::{sleep, Duration}};

use crate::domain::operation::Operation;
use crate::repository::user_repository::UserRepository;
use crate::services::service_user::InMemoryUserRepository;
use crate::usecase::register_user::RegisterUserUseCase;
use crate::controller::user_handler::{find_user_handler, register_user_handler};

#[tokio::main]
async fn main() {
    let repo = Arc::new(InMemoryUserRepository::new());
    // R = Arc<InMemoryUserRepository>
    let usecase = Arc::new(RegisterUserUseCase::new(repo.clone()));

    let mut tasks = JoinSet::new();
    tasks.spawn(start_polling(repo.clone()));   // ← Arc clone

    let app = Router::new()
        // ルートのジェネリックを Arc<…> に合わせる
        .route("/register_user",
               post(register_user_handler::<Arc<InMemoryUserRepository>>))
        .route("/find_user",
               post(find_user_handler::<Arc<InMemoryUserRepository>>))
        .with_state(usecase);

    Server::bind("0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn start_polling<R: UserRepository + 'static>(repo: Arc<R>) {
    let client = Client::new();

    loop {
        match client.get("http://localhost:4000/operations").send().await {
            Ok(res) => match res.json::<Vec<Operation>>().await {
                Ok(ops) => {
                    for op in ops {
                        if op.status == "pending" {
                            // println!("pending op: {:?}", op);
                            // ちゃんとuser_idが見つかれば、スコアをつけて送信させる。
                            match repo.find_by_id(&op.user_id).await {
                                Some(user) => {
                                    // println!("User found: {:?}", user);
                                    let score = rand::thread_rng().gen_range(60..=100);
                                    repo.update_score(&user.id, score).await;
                        
                                    let res = client
                                        .post(format!(
                                            "http://localhost:4000/operations/{}/result",
                                            op.id
                                        ))
                                        .json(&serde_json::json!({
                                            "user_id": user.id,
                                            "score": score,
                                        }))
                                        .send()
                                        .await;
                        
                                    match res {
                                        Ok(response) => {
                                            println!("POST成功: status = {}", response.status());
                                            break
                                        }
                                        Err(e) => {
                                            eprintln!("POST送信失敗: {e:?}");
                                        }
                                    }
                                }
                                None => {
                                    println!("user_id {} が未登録のためスキップ", op.user_id);
                                }
                            }
                        }                        
                    }
                }
                Err(e) => eprintln!("JSON 変換失敗: {e:?}"),
            },
            Err(e) => eprintln!("通信失敗: {e:?}"),
        }

        sleep(Duration::from_secs(5)).await; // 5 秒ごとに再試行
    }
}
