use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user_kits")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(belongs_to, from = "user_id", to = "persona_id")]
    pub user: HasOne<super::users::Entity>,
    #[sea_orm(column_type = "JsonBinary", default = "'{}'")]
    pub kits: Json,
}

impl ActiveModelBehavior for ActiveModel {}
