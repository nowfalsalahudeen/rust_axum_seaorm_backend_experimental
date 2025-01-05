// use crate::config::errors::Error;

pub use self::errors::Error;

pub mod config;
pub mod controllers;

pub mod cache;
pub mod entity;

pub mod errors;

/// Application results options list
pub type Result<T, E = Error> = std::result::Result<T, E>;
