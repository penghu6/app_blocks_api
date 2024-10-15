use crate::domain::entities::user_component::Model as UserComponent;
use crate::domain::repositories::base_repository::BaseRepository;
use async_trait::async_trait;

#[async_trait]
pub trait UserComponentRepository: BaseRepository<UserComponent> {}
