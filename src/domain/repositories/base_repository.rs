use async_trait::async_trait;
use sea_orm::DbErr;

#[async_trait]
pub trait BaseRepository<T> {
    async fn create(&self, entity: T) -> Result<T, DbErr>;
    async fn find_by_id(&self, entity: T,id:i32) -> Result<Option<T>, DbErr>;
    async fn update(&self, entity: T) -> Result<T, DbErr>;
    async fn delete(&self, entity: T) -> Result<(), DbErr>;
    async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<T>, u64), DbErr>;
}

