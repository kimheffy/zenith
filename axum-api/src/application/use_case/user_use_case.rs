use crate::application::repository::user_repo_trait;
use crate::entity::user::RegisterUserRequest;
use anyhow::Context;
use argon2::Argon2;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

fn hash_password(password: &String) -> anyhow::Result<[u8; 32]> {
    let salt = SaltString::generate(&mut OsRng).to_string();
    let argon2 = Argon2::default();
    let mut generated_password = [0u8; 32];
    argon2.hash_password_into(
        password.as_bytes(),
        &salt.as_bytes(),
        &mut generated_password,
    );
    Ok(generated_password)
}

pub async fn register_user_use_case(
    registered_user: &RegisterUserRequest,
    user_repo: &dyn user_repo_trait::UserRepo,
) -> anyhow::Result<()> {
    let hashed_password = hash_password(&registered_user.password)?;

    user_repo
        .register_user(registered_user, hashed_password)
        .await
        .context("failed to register user")?;

    Ok(())
}
