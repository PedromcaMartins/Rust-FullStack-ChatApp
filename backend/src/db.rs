use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub async fn run_migrations(
    rocket: rocket::Rocket<rocket::Build>
) -> rocket::Rocket<rocket::Build> {
    let conn = DbConn::get_one(&rocket).await.expect("database connection");
        conn.run(|c| {
                log::info!("Has pending migration: {:?}",
                    c.has_pending_migration(MIGRATIONS)
                );
                c.run_pending_migrations(MIGRATIONS).expect("can run migrations");
            })
            .await;

    rocket
}