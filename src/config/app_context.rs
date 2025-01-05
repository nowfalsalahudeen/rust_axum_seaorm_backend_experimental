use crate::cache;
use crate::cache::Cache;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use crate::config::Config;

#[derive(Clone)]
pub struct AppContext {
  pub db: DatabaseConnection,
  pub cache: Arc<Cache>,
  pub config: Config,
}

impl AppContext {
  pub fn new(db: DatabaseConnection, cache: Arc<Cache>) -> Self {
    Self {
      db,
      cache,
      config: Config::new(),
    }
  }
}


pub async fn get_app_context() -> AppContext {
  let db = Database::connect("db_url").await.unwrap();
  AppContext::new(
    db,
    Cache::new(cache::drivers::inmem::new()).into()
  )
}
