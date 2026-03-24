use crate::application::repository::user_repo_trait::UserRepo;
use crate::framework::axum::config::AppConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_repo: Arc<dyn UserRepo>,
}
