use crate::domain::user::User;
use crate::repository::user_repository::UserRepository;
// use crate::services::service_user::InMemoryUserRepository;

pub struct RegisterUserUseCase<R: UserRepository> {
    repo: R,
}

//impl RegisterUserUseCase<InMemoryUserRepository> {
//    pub fn new() -> Self {
//        RegisterUserUseCase {
//            repo: InMemoryUserRepository::new(),
//        }
//    }
//}

impl<R: UserRepository> RegisterUserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: String, name: String) {
        let user = User::new(id, name);
        self.repo.save(user).await;
    }

    pub async fn find(&self, id: String) -> Option<User> {
        self.repo.find_by_id(&id).await
    }
}