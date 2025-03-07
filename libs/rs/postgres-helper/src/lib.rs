use std::time::Duration;

use log::info;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod cargo;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    info!("Initializing database connection");
    let pool = PgPoolOptions::new()
        .min_connections(0)
        .max_connections(16)
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(database_url)
        .await?;

    Ok(pool)
}

#[macro_export]
macro_rules! migrate {
    ($uri:expr, $path:literal) => {
        use sqlx::{migrate::MigrateDatabase, Connection};

        if !sqlx::Postgres::database_exists($uri).await? {
            log::info!("Creating database...");
            sqlx::Postgres::create_database($uri).await?;
        }

        log::info!("Applying migrations...");

        let mut conn = sqlx::PgConnection::connect($uri).await?;
        sqlx::migrate!($path)
            .set_locking(false)
            .run(&mut conn)
            .await
            .expect("Error while running database migrations!");
    };
}
