pub mod app_context;
pub mod db;
pub mod routes_config;

pub mod format;
// export only these functions from tasks_routes
// pub use tasks_routes::tasks_routes;
#[derive(Clone)]
pub struct Config {
    pub debug_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Constructs a `Config` instance, determining the build mode.
    pub fn new() -> Self {
        Self {
            debug_mode: cfg!(debug_assertions),
        }
    }
}