use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};
use uuid::Uuid;

use crate::{
    entity::user::{ActiveModel, Model},
    infra::db::get_db,
};

pub async fn add_user() -> Result<Model, DbErr> {
    let db = get_db().await;
    let user_model: ActiveModel = ActiveModel {
        name: Set("test".to_owned()),
        email: Set("test@example.com".to_owned()),
        password: Set("password".to_owned()),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let user = user_model.insert(&db).await?;

    Ok(user)
}
