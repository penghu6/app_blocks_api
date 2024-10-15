use crate::domain::model::version_management_model::VersionManagementModel;
use crate::domain::repositories::version_management_repository::VersionManagementRepository;
use sea_orm::DbErr;
use std::sync::Arc;
use di::injectable;

#[injectable]
pub struct VersionManagementService {
    repository: Arc<dyn VersionManagementRepository>,
}

impl VersionManagementService {
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

