use std::any::Any;
use std::sync::Arc;
use di::{existing_as_self, singleton, Injectable, Ref, ServiceCollection, ServiceProvider};
use lazy_static::lazy_static;
use crate::application::services::user_component_service::UserComponentService;
use crate::infrastructure::persistence::user_component_repository_impl::UserComponentRepositoryImpl;
use crate::config::DatabaseManager;

lazy_static! {
    pub static ref DI: ServiceProvider = {
        let provider = ServiceCollection::new()
            .add(existing_as_self(DatabaseManager::new()))
            .add(UserComponentService::singleton())
            .add(UserComponentRepositoryImpl::singleton())
            .build_provider()
            .expect("Failed to build DI provider.");

        provider
    };
}

pub fn get_it<T: Any + ?Sized>() -> Ref<T> {
  DI.get_required::<T>()
}
