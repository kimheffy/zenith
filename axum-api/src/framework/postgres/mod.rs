pub mod persistence;
pub mod setup;

use crate::framework::postgres::{persistence::PostgresPersistence, setup::init_db};

pub async fn postgres_persistence() -> anyhow::Result<PostgresPersistence> {
    let pool = init_db().await?;
    let postgres_persistence = PostgresPersistence::new(pool);
    Ok(postgres_persistence)
}
