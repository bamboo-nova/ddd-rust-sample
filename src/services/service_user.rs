use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::user::User;
use crate::repository::user_repository::UserRepository;

pub struct InMemoryUserRepository {
    store: Arc<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepository { 
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: User) {
        self.store.write().await.insert(user.id.clone(), user);
    }

    async fn find_by_id(&self, id: &str) -> Option<User> {
        self.store.read().await.get(id).cloned()
    }

    async fn update_score(&self, id: &str, score: u32) {
        let mut store = self.store.write().await;
        // 先にcloneして、User のコピーを作ることで store の参照を早期に解放
        // でないと、同時に mutable と immutable な借用を行っているのでエラー
        if let Some(user) = store.get(id).cloned() {
            store.insert(id.to_string(), user.with_score(score));
        }
    }
}