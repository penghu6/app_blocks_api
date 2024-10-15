use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "version_management")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub date: DateTimeWithTimeZone,
    pub version: String,
    pub code_content: String,
    pub name: String,
    pub creation_date: DateTimeWithTimeZone,
    pub image_preview: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}