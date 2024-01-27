use sea_orm::*;

use crate::connect_database;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "reputation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub reputation: i32,
}

impl Model {
    pub fn new(id: i64, reputation: i32, name: String) -> Self {
        Self {
            id,
            name,
            reputation,
        }
    }

    pub async fn create(reputation: Model) -> Result<(), sea_orm::error::DbErr> {
        let db = connect_database().await;
        let active_model: ActiveModel = reputation.into();
        let _ = active_model.insert(&db).await?;
        Ok(())
    }

    pub async fn find_user_by_id(id: i64) -> Option<Model> {
        let user = Entity::find_by_id(id)
            .one(&connect_database().await)
            .await
            .ok();
    
        user?
    }

    pub async fn get_top_users(limit: usize) -> Result<Vec<Model>, sea_orm::error::DbErr> {
        let users = Entity::find()
            .order_by_desc(Column::Reputation)
            .limit(Some(limit as u64))
            .all(&connect_database().await)
            .await?;
        Ok(users)
    }

    pub async fn get_bot_users(limit: usize) -> Result<Vec<Model>, sea_orm::error::DbErr> {
        let users = Entity::find()
            .order_by_asc(Column::Reputation)
            .limit(Some(limit as u64))
            .all(&connect_database().await)
            .await?;
        Ok(users)
    }

pub async fn increment_reputation(id: i64) -> Result<(), sea_orm::error::DbErr> {
    let db = connect_database().await;
    let user = Entity::find_by_id(id).one(&db).await?;

    if let Some(user_model) = user {
        let mut active_model: ActiveModel = user_model.into();
        active_model.reputation = Set(active_model.reputation.unwrap() + 1);
        let _ = active_model.update(&db).await?;
    }

    Ok(())
}

pub async fn decrement_reputation(id: i64) -> Result<(), sea_orm::error::DbErr> {
    let db = connect_database().await;
    let user = Entity::find_by_id(id).one(&db).await?;

    if let Some(user_model) = user {
        let mut active_model: ActiveModel = user_model.into();
        active_model.reputation = Set(active_model.reputation.unwrap() - 1);
        let _ = active_model.update(&db).await?;
    }

    Ok(())
}


}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
