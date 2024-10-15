use crate::domain::model::version_management_model::VersionManagementModel;
use crate::domain::repositories::version_management_repository::VersionManagementRepository;
use sea_orm::DbErr;
use std::sync::Arc;

pub struct VersionManagementService<R: VersionManagementRepository> {
    repository: Arc<R>,
}

impl<R: VersionManagementRepository> VersionManagementService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, model: VersionManagementModel) -> Result<VersionManagementModel, DbErr> {
        self.repository.create(model).await
    }

    pub async fn update(&self, model: VersionManagementModel) -> Result<VersionManagementModel, DbErr> {
        self.repository.update(model).await
    }

    pub async fn delete(&self, model: VersionManagementModel) -> Result<(), DbErr> {
        self.repository.delete(model).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<VersionManagementModel>, DbErr> {
        self.repository.find_by_id(id).await
    }
}

