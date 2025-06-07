use std::sync::Arc;
use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User);
    async fn find_by_id(&self, id: &str) -> Option<User>;
    async fn update_score(&self, id: &str, score: u32);
}

#[async_trait::async_trait]
impl<T> UserRepository for Arc<T>
where
    T: UserRepository + Send + Sync,
{
    async fn save(&self, user: User) {
        (**self).save(user).await
    }

    async fn find_by_id(&self, id: &str) -> Option<User> {
        (**self).find_by_id(id).await
    }

    async fn update_score(&self, id: &str, score: u32) {
        (**self).update_score(id, score).await
    }
}
