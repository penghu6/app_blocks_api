
use async_trait::async_trait;
use sea_orm::DbErr;
use crate::domain::entities::user_component;
use crate::domain::model::user_component_model::UserComponentModel;

#[async_trait]
pub trait UserComponentRepository
where
  Self: Send + Sync,
{
  async fn create(&self, model: UserComponentModel) -> Result<UserComponentModel, DbErr>;
  async fn update(&self, model: UserComponentModel) -> Result<UserComponentModel, DbErr>;
  async fn delete(&self, model: UserComponentModel) -> Result<(), DbErr>;
  async fn find_by_id(&self, id: i32) -> Result<Option<UserComponentModel>, DbErr>;
  async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<user_component::Model>, u64), DbErr>;
}