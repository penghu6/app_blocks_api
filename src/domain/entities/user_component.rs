use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_components")]
pub struct Model {
  #[sea_orm(primary_key)]
  #[sea_orm(column_name = "id", comment = "自增主键")]
  pub id: i32,
  #[sea_orm(column_name = "react_code", comment = "React组件代码")]
  pub react_code: String,
  #[sea_orm(column_name = "created_at", comment = "创建时间")]
  pub created_at: DateTimeWithTimeZone,
  #[sea_orm(column_name = "updated_at", comment = "修改时间")]
  pub updated_at: DateTimeWithTimeZone,
  #[sea_orm(column_name = "version", comment = "版本号")]
  pub version: i32,
  #[sea_orm(column_name = "image_url", comment = "组件图片地址")]
  pub image_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
