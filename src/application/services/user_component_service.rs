use sea_orm::DbErr;
use crate::domain::entities::user_component::Model as UserComponent;
use crate::domain::repositories::user_component_repository::UserComponentRepository;
use crate::application::services::base_service::BaseService;

pub type UserComponentService<R> = BaseService<UserComponent, R>;

impl<R: UserComponentRepository> UserComponentService<R> {
    // 在这里可以添加特定于 UserComponent 的方法
}
