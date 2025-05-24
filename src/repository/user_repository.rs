use crate::domain::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User);
    async fn find_by_id(&self, id: &str) -> Option<User>;
}