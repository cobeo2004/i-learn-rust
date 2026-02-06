use crate::utils;
use sea_orm::{Database, DatabaseConnection};

pub async fn get_db() -> DatabaseConnection {
    let db: DatabaseConnection = Database::connect(utils::env::get("DATABASE_URL"))
        .await
        .unwrap();

    db.ping().await.expect("Can not ping the database");

    return db;
}
