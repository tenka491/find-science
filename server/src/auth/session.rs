use uuid::Uuid;
use sqlx::SqlitePool;
use anyhow::Result;

pub async fn create_session(
    pool: &SqlitePool,
    user_id: Option<i64>,
) -> Result<String> {
    let session_id = Uuid::new_v4().to_string();

    sqlx::query!(
        r#"
        INSERT INTO sessions (id, data, expires_at)
        VALUES (?, ?, strftime('%s','now') + 60 * 60 * 24 * 7)
        "#,
        session_id,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(session_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{ sqlite::SqlitePoolOptions };
    use tokio;

    #[tokio::test]
    async fn test_create_session() {
        // in-memory SQLite DB
        let pool = SqlitePoolOptions::new()
            .connect(":memory:")
            .await
            .expect("failed to create pool");

        sqlx::query(
            "CREATE TABLE sessions (id TEXT PRIMARY KEY, data TEXT NOT NULL, expires_at INTEGER)"
        )
        .execute(&pool)
        .await
        .expect("failed to create table");

        let user_id = Some(1_i64);
        let session_id = create_session(&pool, user_id).await.expect("failed to create session");

        assert!(!session_id.is_empty(), "session id should not be empty");
    }
}
