use crate::application::repository::user_repo_trait::UserRepo;
use crate::application::service::authentication_service::AuthenticationServiceTrait;
use crate::framework::axum::config::AppConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub authentication_service: Arc<dyn AuthenticationServiceTrait>,
    pub user_repo: Arc<dyn UserRepo>,
}
