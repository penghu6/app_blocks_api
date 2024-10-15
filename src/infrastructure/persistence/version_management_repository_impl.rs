use crate::Arc;
use async_trait::async_trait;
use di::injectable;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter};
use crate::config::DatabaseManager;
use crate::domain::entities::{user_component, version_management};
use sea_orm::{ActiveModelTrait};
use crate::domain::model::version_management_model::VersionManagementModel;
use crate::domain::repositories::version_management_repository::VersionManagementRepository;

#[injectable(VersionManagementRepository)]
pub struct VersionManagementRepositoryImpl {
    client: Arc<DatabaseManager>,
}

#[async_trait]
impl VersionManagementRepository for VersionManagementRepositoryImpl {
    async fn create(&self, model: VersionManagementModel) -> Result<VersionManagementModel, DbErr> {
        let conn = self.client.get().await;
        let item: version_management::Model = model.into();
        let result = item.into_active_model().insert(conn.as_ref()).await?;
        Ok(result.last_insert_id.into())
    }

    async fn update(&self, model: VersionManagementModel) -> Result<VersionManagementModel, DbErr> {
        let conn = self.client.get().await;
        let item: version_management::Model = model.into();
        let mut am = item.into_active_model().into_active_model();
        let result = am.update(conn.as_ref()).await?;
        Ok(result.into())
    }

    async fn delete(&self, model: VersionManagementModel) -> Result<(), DbErr> {

        let item: version_management::Model = model.into();
        let conn = self.client.get().await;
        item.into_active_model().delete(conn.as_ref()).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<VersionManagementModel>, DbErr> {
        let conn = self.client.get().await;
        let result = version_management::Entity::find_by_id(id).one(conn.as_ref()).await?.map(|item| item.into());
        Ok(result)
    }

    async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<version_management::Model>, u64), DbErr> {

        let conn = self.client.get().await;
        let paginator = version_management::Entity::find().paginate(conn.as_ref(), per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        Ok((items, total))
    }
}

