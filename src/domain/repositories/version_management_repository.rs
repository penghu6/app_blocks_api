use async_trait::async_trait;
use sea_orm::DbErr;
use crate::domain::entities::version_management;
use crate::domain::model::version_management_model::VersionManagementModel;

#[async_trait]
pub trait VersionManagementRepository
where
    Self: Send + Sync,
{
    async fn create(&self, model: VersionManagementModel) -> Result<VersionManagementModel, DbErr>;
    async fn update(&self, model: VersionManagementModel) -> Result<VersionManagementModel, DbErr>;
    async fn delete(&self, model: VersionManagementModel) -> Result<(), DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<VersionManagementModel>, DbErr>;
    async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<version_management::Model>, u64), DbErr>;
}

