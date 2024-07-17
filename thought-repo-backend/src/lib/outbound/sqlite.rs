use std::str::FromStr;

use anyhow::{anyhow, Context};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{SqlitePool, Transaction};
use uuid::Uuid;

use crate::domain::blog::models::blog::{Blog, CreateBlogError, CreateBlogRequest};
use crate::domain::blog::ports::BlogRepository;

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> Result<Sqlite, anyhow::Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }

    async fn save_author(
        &self,
        tx: &mut Transaction<'_, sqlx::Sqlite>,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        Ok(id)
    }
}

impl BlogRepository for Sqlite {
    async fn create_blog(&self, req: &CreateBlogRequest) -> Result<Blog, CreateBlogError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start SQLite transaction")?;

        self.save_author(&mut tx).await.map_err(|e| {
            if is_unique_constraint_violation(&e) {
                CreateBlogError::Unknown(anyhow!("blog name must be unique"))
            } else {
                anyhow!(e)
                    .context(format!("failed to save blog with name"))
                    .into()
            }
        })?;

        tx.commit()
            .await
            .context("failed to commit SQLite transaction")?;

        Ok(Blog)
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}
