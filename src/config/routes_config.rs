use crate::config::app_context::AppContext;
use axum::Router;

#[derive(Clone)]
pub struct AppRoutes {
  prefix: Option<String>,
  routes: Vec<Routes>,
}

impl AppRoutes {
  /// Creates a new [`AppRoutes`] instance with default settings.
  pub fn with_default_routes() -> Self {
    Self {
      prefix: None,
      routes: Vec::new(),
    }
  }

  /// Adds a new route to the collection.
  pub fn add_route(mut self, route: Routes) -> Self {
    self.routes.push(route);
    self
  }

  /// Sets a prefix for all config.
  pub fn prefix(mut self, prefix: &str) -> Self {
    self.prefix = Some(prefix.to_string());
    self
  }

  /// Converts the `AppRoutes` into an Axum `Router`.
  pub fn into_router(self, ctx: &AppContext) -> Router {
    let mut router = Router::new();

    for route in self.routes {
      let prefix = self.prefix.as_deref().unwrap_or("");
      let route_prefix = route.prefix.as_deref().unwrap_or("");
      let full_prefix = format!("{}{}", prefix, route_prefix);

      let mut route_router = Router::new();
      for handler in route.handlers {
        route_router = route_router.route(&handler.uri, handler.method);
      }

      router = router.nest(&full_prefix, route_router);
    }

    router.with_state(ctx.clone())
  }
}

#[derive(Clone, Default, Debug)]
pub struct Routes {
  pub prefix: Option<String>,
  pub handlers: Vec<Handler>,
}

#[derive(Clone, Default, Debug)]
pub struct Handler {
  pub uri: String,
  pub method: axum::routing::MethodRouter<AppContext>,
  pub actions: Vec<axum::http::Method>,
}

impl Routes {
  /// Creates a new [`Routes`] instance with default settings.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets a prefix for the config.
  pub fn at(prefix: &str) -> Self {
    Self {
      prefix: Some(prefix.to_string()),
      ..Self::default()
    }
  }

  /// Adds a new route handler.
  pub fn add(mut self, uri: &str, method: axum::routing::MethodRouter<AppContext>) -> Self {
    self.handlers.push(Handler {
      uri: uri.to_owned(),
      method,
      actions: vec![],
    });
    self
  }

  /// Sets a prefix for the config.
  pub fn prefix(mut self, uri: &str) -> Self {
    self.prefix = Some(uri.to_owned());
    self
  }
}
