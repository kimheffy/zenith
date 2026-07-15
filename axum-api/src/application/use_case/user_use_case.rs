use crate::application::repository::user_repo_trait;
use crate::application::service::authentication_service;
use crate::entity::error::AppError;
use crate::entity::user::{RegisterUserRequest, SignUserInRequest};
use crate::framework::axum::config;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

fn hash_password(password: &String) -> anyhow::Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|pass| pass.to_string())
        .map_err(|_| AppError::PasswordHashFailed)
}

pub async fn register_user_use_case(
    registered_user: &RegisterUserRequest,
    user_repo: &dyn user_repo_trait::UserRepo,
    auth_service: &dyn authentication_service::AuthenticationServiceTrait,
    config: &config::AppConfig,
) -> anyhow::Result<String, AppError> {
    let hashed_password = hash_password(&registered_user.password)?;

    let created_uuid = user_repo
        .register_user(registered_user, &hashed_password.as_bytes())
        .await?;
    auth_service.create_session(created_uuid, config.jwt_secret.as_bytes())
}

pub async fn sign_user_in_use_case(
    user: &SignUserInRequest,
    user_repo: &dyn user_repo_trait::UserRepo,
    auth_service: &dyn authentication_service::AuthenticationServiceTrait,
    config: &config::AppConfig,
) -> anyhow::Result<String, AppError> {
    let found_user = user_repo.find_user_by_email(&user.email).await;
    if found_user.is_err() {
        // TODO: create a new AppError (Failed to find account with email)
        return Err(AppError::DatabaseOperationError);
    }

    let found_user = found_user.unwrap();

    let hashed_password = hash_password(&user.password)?.to_string();
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();

    let verify_password =
        Argon2::default().verify_password(&user.password.as_bytes(), &parsed_hash);

    if verify_password.is_err() {
        // TODO: throw something else
        return Err(AppError::PasswordHashFailed);
    }

    auth_service.create_session(found_user.id, config.jwt_secret.as_bytes())
}
