use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::ConnectOptions;

use crate::Db;

pub async fn setup_database() -> Result<Db> {
    let migrations = sqlx::migrate::Migrator::new(Path::new("./migrations")).await?;

    let database_url = env::var("DATABASE_URL")?;

    let mut connection_options = SqliteConnectOptions::from_str(&database_url)?;
    connection_options.disable_statement_logging();

    let pool = Arc::new(
        SqlitePoolOptions::new()
            .max_connections(15)
            .connect_with(connection_options)
            .await?,
    );

    migrations.run(&*pool).await?;

    Ok(pool)
}
