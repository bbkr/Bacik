use sqlx::{prelude::FromRow, sqlite::SqliteQueryResult, SqliteConnection};

#[derive(Debug, FromRow)]
pub struct Session {
    id: u64,
    name: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>
}

impl Session {
    pub async fn init ( connection: &mut SqliteConnection ) -> Result<SqliteQueryResult, sqlx::Error> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS "sessions" (
                "id" INTEGER NOT NULL,
                "name" TEXT,
                "created_at" INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY("id" AUTOINCREMENT)
            )
        "#;
        sqlx::query(query).execute(&mut *connection).await
    }

    pub async fn create ( connection: &mut SqliteConnection, name: Option<&str> ) -> Result<Self, sqlx::Error> {
        let query = r#"
            INSERT INTO "sessions" ("name")
            VALUES($1)
        "#;
        let result = sqlx::query(query).bind(&name).execute(&mut *connection).await?;
        result.last_insert_rowid();

        Self::read(connection, result.last_insert_rowid()).await
    }

    pub async fn read ( connection: &mut SqliteConnection, id: i64 ) -> Result<Self, sqlx::Error> {
        let query = sqlx::query_as::<_, Session>(
            r#"
                SELECT "id", "name", "created_at"
                FROM "sessions"
                WHERE "id" = $1
            "#
        );
        query.bind(id).fetch_one(&mut *connection).await
    }
}
