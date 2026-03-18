use crate::application::repository::user_repo_trait::UserRepo;
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<dyn UserRepo>,
}
