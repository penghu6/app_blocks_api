use std::sync::Arc;
use sea_orm::DbErr;
use chrono::Utc;
use di::injectable;
use crate::domain::entities::user_component::Model as UserComponent;
use crate::domain::model::user_component_model::UserComponentModel;
use crate::domain::repositories::user_component_repository::UserComponentRepository;

#[injectable]
pub struct UserComponentService {
  repo: Arc<dyn UserComponentRepository>,
}

impl UserComponentService {
  pub async fn create_user_component(&self, model: UserComponentModel) -> Result<UserComponentModel, DbErr> {
    let now = Utc::now();
    let new_component = UserComponentModel {
      id: 0,
      react_code: model.react_code.clone(),
      created_at: now.into(),
      updated_at: now.into(),
      version: 1,
      image_url: model.image_url.clone(),
    };
    self.repo.create(new_component).await
  }

  // 其他方法...
}
