use chrono::{DateTime, Utc};
use sea_orm::{
  ActiveEnum, ColumnDef, ColumnType, ColumnTypeTrait, DbErr, DeriveActiveEnum, DeriveDisplay, EnumIter, FromQueryResult,
};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Convert, FromQueryResult)]
#[convert(from = "crate::domain::entities::user_component::Model")]
#[convert(into = "crate::domain::entities::user_component::Model")]
pub struct UserComponentModel {
  pub id: i32,
  pub react_code: String,
  pub created_at: DateTimeWithTimeZone,
  pub updated_at: DateTimeWithTimeZone,
  pub version: i32,
  pub image_url: Option<String>,
}