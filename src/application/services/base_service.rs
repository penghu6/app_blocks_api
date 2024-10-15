use crate::domain::repositories::base_repository::BaseRepository;
use sea_orm::DbErr;
use std::marker::PhantomData;

pub struct BaseService<T, R: BaseRepository<T>> {
    repository: R,
    _phantom: PhantomData<(T)>,
}

impl<T, R: BaseRepository<T>> BaseService<T, R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            _phantom: PhantomData,
        }
    }

    pub async fn create(&self, entity: T) -> Result<T, DbErr> {
        self.repository.create(entity).await
    }

    pub async fn get(&self, entity: T,id:i32) -> Result<Option<T>, DbErr> {
        self.repository.find_by_id(entity,id).await
    }

    pub async fn update(&self, entity: T) -> Result<T, DbErr> {
        self.repository.update(entity).await
    }

    pub async fn delete(&self, entity: T) -> Result<(), DbErr> {
        self.repository.delete(entity).await
    }

    pub async fn list(&self, page: u64, per_page: u64) -> Result<(Vec<T>, u64), DbErr> {
        self.repository.find_all(page, per_page).await
    }
}
