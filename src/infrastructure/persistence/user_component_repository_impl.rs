use async_trait::async_trait;
use sea_orm::{DbErr, PaginatorTrait};
use std::sync::Arc;
use di::injectable;
use crate::config::DatabaseManager;
use crate::domain::entities::{user_component};
use crate::domain::model::user_component_model::UserComponentModel;
use crate::domain::repositories::user_component_repository::UserComponentRepository;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};

#[injectable(UserComponentRepository)]
pub struct UserComponentRepositoryImpl {
  client: Arc<DatabaseManager>,
}

#[async_trait]
impl UserComponentRepository for UserComponentRepositoryImpl {
  async fn create(&self, entity: UserComponentModel) -> Result<UserComponentModel, DbErr> {
    let item: user_component::Model = entity.into();
    let conn = self.client.get().await;
    let result = item.into_active_model().insert(conn.as_ref()).await?;
    Ok(result.into())
  }

  async fn update(&self, entity: UserComponentModel) -> Result<UserComponentModel, DbErr> {
    let conn = self.client.get().await;
    let item: user_component::Model = entity.into();
    let mut am = item.into_active_model().into_active_model();
    let result = am.update(conn.as_ref()).await?;
    Ok(result.into())
  }

  async fn delete(&self, entity: UserComponentModel) -> Result<(), DbErr> {
    let item: user_component::Model = entity.into();
    let conn = self.client.get().await;
    item.into_active_model().delete(conn.as_ref()).await?;
    Ok(())
  }

  async fn find_by_id(&self, id: i32) -> Result<Option<UserComponentModel>, DbErr> {
    let conn = self.client.get().await;
    let result = user_component::Entity::find_by_id(id).one(conn.as_ref()).await?.map(|item| item.into());
    Ok(result)
  }

  async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<user_component::Model>, u64), DbErr> {
    let conn = self.client.get().await;
    let paginator = user_component::Entity::find().paginate(conn.as_ref(), per_page);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page - 1).await?;
    Ok((items, total))
  }
}
