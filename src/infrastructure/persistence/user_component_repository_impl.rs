use crate::domain::entities::user_component::{Entity as UserComponentEntity, Model as UserComponent, ActiveModel as UserComponentActiveModel};
use crate::domain::repositories::base_repository::BaseRepository;
use crate::domain::repositories::user_component_repository::UserComponentRepository;
use crate::infrastructure::persistence::base_repository_impl::BaseRepositoryImpl;
use async_trait::async_trait;
use sea_orm::DbErr;
use std::sync::Arc;
use di::injectable;
use crate::config::DatabaseManager;

pub struct UserComponentRepositoryImpl {
    base: BaseRepositoryImpl<UserComponentEntity, UserComponentActiveModel>,
}

#[async_trait]
impl UserComponentRepository for UserComponentRepositoryImpl {}

#[async_trait]
impl BaseRepository<UserComponent> for UserComponentRepositoryImpl {
    async fn create(&self, entity: UserComponent) -> Result<UserComponent, DbErr> {
        self.base.create(entity).await
    }

    async fn find_by_id(&self,entity: UserComponent,id:i32) -> Result<Option<UserComponent>, DbErr> {
        self.base.find_by_id(entity,id).await
    }

    async fn update(&self, entity: UserComponent) -> Result<UserComponent, DbErr> {
        self.base.update(entity).await
    }

    async fn delete(&self, entity: UserComponent) -> Result<(), DbErr> {
        self.base.delete(entity).await
    }

    async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<UserComponent>, u64), DbErr> {
        self.base.find_all(page, per_page).await
    }
}
