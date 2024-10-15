use std::sync::Arc;
use std::marker::PhantomData;
use async_trait::async_trait;
use di::injectable;
use sea_orm::{EntityTrait, ActiveModelTrait, IntoActiveModel, ActiveModelBehavior, ModelTrait, DbErr, PaginatorTrait, PrimaryKeyTrait, DatabaseConnection};
use crate::config::DatabaseManager;
use crate::domain::repositories::base_repository::BaseRepository;

pub struct BaseRepositoryImpl<E, A>
where
  E: EntityTrait,
  A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send + Sync + std::convert::From<<E as sea_orm::EntityTrait>::Model>,
{
    client: Arc<DatabaseManager>,
    _phantom: PhantomData<(E, A)>,
}

#[async_trait]
impl<E, A> BaseRepository<E::Model> for BaseRepositoryImpl<E, A>
where
    E: EntityTrait,
    A: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send + Sync + std::convert::From<<E as sea_orm::EntityTrait>::Model>,
    E::Model: IntoActiveModel<A>+ Send + Sync,
    <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
{
    async fn create(&self, entity: E::Model) -> Result<E::Model, DbErr> {

        let item: A = entity.into();
        let conn = self.client.get().await;
        let result = item.into_active_model().insert(conn.as_ref()).await?;
        Ok(result)
    }

    async fn find_by_id(&self,entity: E::Model,id:i32) -> Result<Option<E::Model>, DbErr> {
        let active_model: A = entity.into();
        let conn = self.client.get().await;
        let result = E::find_by_id(12).one(conn.as_ref()).await?.map(|item| item.into());
        Ok(result)
    }

    async fn update(&self, entity: E::Model) -> Result<E::Model, DbErr> {
        let conn = self.client.get().await;
        let item: A  = entity.into_active_model();
        let mut am = item.into_active_model().into_active_model();
        let result = am.update(conn.as_ref()).await?;
        Ok(result.into())
    }

    async fn delete(&self,entity: E::Model) -> Result<(), DbErr> {
        let conn = self.client.get().await;
        let active_model: A  = entity.into_active_model();
        active_model.delete(conn.as_ref()).await?;
        Ok(())
    }

    async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<E::Model>, u64), DbErr> {
        let conn = self.client.get().await;
        let paginator = E::find().paginate(conn.as_ref(), per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        Ok((items, total))
    }
}
