use chrono::{DateTime, Utc};
use protos::admin::AdminResponse;
use sqlx::{query_file, query_file_as, Error, Executor, Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug)]
pub struct DBAdmin {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub last_password_change: DateTime<Utc>,
    pub deleted: bool,
}
impl DBAdmin {
    pub async fn insert(self, transaction: &mut Transaction<'_, Postgres>) -> Result<Self, Error> {
        Ok(query_file_as!(
            Self,
            "db/queries/insert.sql",
            self.id,
            self.email,
            self.password_hash,
        )
        .fetch_one(&mut **transaction)
        .await?)
    }

    pub async fn get_by_id<'a, E>(id: Uuid, executor: E) -> Result<Option<Self>, Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_file_as!(Self, "db/queries/get_by_id.sql", id)
            .fetch_optional(executor)
            .await?)
    }

    pub async fn get_by_email<'a, E>(email: &str, executor: E) -> Result<Option<Self>, Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_file_as!(Self, "db/queries/get_by_email.sql", email)
            .fetch_optional(executor)
            .await?)
    }

    pub async fn patch(
        id: Uuid,
        email: Option<&str>,
        password_hash: Option<&str>,
        last_password_change: Option<DateTime<Utc>>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, Error> {
        Ok(query_file_as!(
            Self,
            "db/queries/patch.sql",
            id,
            email,
            password_hash,
            last_password_change,
        )
        .fetch_optional(&mut **transaction)
        .await?)
    }

    pub async fn delete(
        id: Uuid,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), Error> {
        query_file!("db/queries/delete.sql", id)
            .fetch_optional(&mut **transaction)
            .await?;
        Ok(())
    }
}
impl Into<AdminResponse> for DBAdmin {
    fn into(self) -> AdminResponse {
        AdminResponse {
            id: self.id.to_string(),
            email: self.email,
        }
    }
}
