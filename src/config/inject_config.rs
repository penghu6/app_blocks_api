use std::any::Any;
use di::{existing_as_self, singleton, Injectable, Ref, ServiceCollection, ServiceProvider};
use lazy_static::lazy_static;
use crate::config::DatabaseManager;

lazy_static! {
  pub static ref DI: ServiceProvider = {
    let Ok(provider) = ServiceCollection::new()
     .add(existing_as_self(DatabaseManager::new()))
      //.add(InMemoryUserRepository::singleton())
      // .add(singleton().from(|sp|{
      // Arc::new(UserService::new(
      //   sp.get_required::<InMemoryUserRepository>(),
      // ))}))
      .build_provider()
    else {
      panic!("Failed to build DI provider.")
    };

    provider
  };
}

pub fn get_it<T: Any + ?Sized>() -> Ref<T> {
  DI.get_required::<T>()
}
