use chrono::{DateTime, Utc};
use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use struct_convert::Convert;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Convert, FromQueryResult)]
#[convert(from = "crate::domain::entities::version_management::Model")]
#[convert(into = "crate::domain::entities::version_management::Model")]
pub struct VersionManagementModel {
    pub id: i32,
    pub date: DateTimeWithTimeZone,
    pub version: String,
    pub code_content: String,
    pub name: String,
    pub creation_date: DateTimeWithTimeZone,
    pub image_preview: Option<String>,
}

