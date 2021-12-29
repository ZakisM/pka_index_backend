use serde::Serialize;

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(transparent)]
pub struct SqliteI32(pub i32);
