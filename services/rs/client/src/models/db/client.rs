use chrono::{DateTime, Utc};
use convertions::datetime_into_timestamp;
use protos::client::{ClientMeta, ClientResponse};
use sqlx::{query_file, query_file_as, Error, Executor, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DBClient {
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub patronymic: String,
    pub email: String,
    pub password_hash: String,
    pub last_password_change: DateTime<Utc>,
    pub send_notifications: bool,
    pub is_internal: bool,
    pub verified: bool,
    pub deleted: bool,
}
impl DBClient {
    pub async fn insert(self, transaction: &mut Transaction<'_, Postgres>) -> Result<Self, Error> {
        Ok(query_file_as!(
            Self,
            "db/queries/insert.sql",
            self.id,
            self.name,
            self.surname,
            self.patronymic,
            self.email,
            self.password_hash,
            self.send_notifications,
            self.is_internal,
            self.verified
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

    pub async fn get_multiple<'a, E>(ids: Vec<Uuid>, executor: E) -> Result<Vec<Self>, Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_file_as!(Self, "db/queries/get_multiple.sql", &ids)
            .fetch_all(executor)
            .await?)
    }

    pub async fn patch(
        id: Uuid,
        name: Option<&str>,
        surname: Option<&str>,
        patronymic: Option<&str>,
        email: Option<&str>,
        password_hash: Option<&str>,
        last_password_change: Option<DateTime<Utc>>,
        send_notifications: Option<bool>,
        is_internal: Option<bool>,
        verified: Option<bool>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, Error> {
        Ok(query_file_as!(
            Self,
            "db/queries/patch.sql",
            id,
            name,
            surname,
            patronymic,
            email,
            password_hash,
            last_password_change,
            send_notifications,
            is_internal,
            verified
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
impl Into<ClientResponse> for DBClient {
    fn into(self) -> ClientResponse {
        ClientResponse {
            id: self.id.to_string(),
            meta: ClientMeta {
                name: self.name,
                surname: self.surname,
                patronymic: self.patronymic,
                email: self.email,
                send_notifications: self.send_notifications,
                is_internal: self.is_internal,
            },
            last_password_cgange: datetime_into_timestamp(self.last_password_change),
            verified: self.verified,
        }
    }
}
