use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use rocket_sync_db_pools::diesel;

use crate::db::models::UserCreateOrUpdate;
use crate::utils;

pub(crate) mod models;

pub(crate) mod schema;
pub(crate) mod table_management_rule_helper;
pub(crate) mod table_user_helper;

#[database("diesel")]
pub(crate) struct RocketDb(diesel::SqliteConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("db/diesel/migrations");

    RocketDb::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
        .await;

    rocket
}

async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    RocketDb::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| {
            let size = table_user_helper::create(conn, &UserCreateOrUpdate {
                name: "admin".to_string(),
                password: utils::encrypt_password("password"),
                permission: "admin".to_string(),
                email: None,
                avatar: None,
            });
            tracing::info!("create admin user, size = {}",size);
        })
        .await;

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(RocketDb::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .attach(AdHoc::on_ignite("Init DB", init_db))
    })
}

pub fn establish_db_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
