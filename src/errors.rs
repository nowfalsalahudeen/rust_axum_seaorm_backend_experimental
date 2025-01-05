//! # Application Error Handling

use crate::Result;
use axum::extract::FromRequest;
use axum::{
  extract::rejection::JsonRejection,
  http::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    method::InvalidMethod,
    StatusCode,
  },
};
use axum_core::__private::tracing;
use axum_core::response::{IntoResponse, Response};
use serde::Serialize;

/*
backtrace principles:
- use a plan wrapper variant with no 'from' conversion
- hand-code "From" conversion and force capture there with 'bt', which
  will wrap and create backtrace only if RUST_BACKTRACE=1.
costs:
- when RUST_BACKTRACE is not set, we don't pay for the capture, and we don`t pay for printing.

 */
impl From<serde_json::Error> for Error {
  fn from(val: serde_json::Error) -> Self {
    Self::JSON(val).bt()
  }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{inner}\n{backtrace}")]
  WithBacktrace {
    inner: Box<Self>,
    backtrace: Box<std::backtrace::Backtrace>,
  },

  #[error("{0}")]
  Message(String),

  #[error(
    "error while running worker: no queue provider populated in context. Did you configure \
         BackgroundQueue and connection details in `queue` in your config file?"
  )]
  QueueProviderMissing,

  #[error("task not found: '{0}'")]
  TaskNotFound(String),

  // #[error(transparent)]
  // Scheduler(#[from] crate::scheduler::Error),
  #[error(transparent)]
  Axum(#[from] axum::http::Error),

  // #[error(transparent)]
  // Tera(#[from] tera::Error),
  #[error(transparent)]
  JSON(serde_json::Error),

  #[error(transparent)]
  JsonRejection(#[from] JsonRejection),

  // #[error("cannot parse `{1}`: {0}")]
  // YAMLFile(#[source] serde_yaml::Error, String),

  // #[error(transparent)]
  // YAML(#[from] serde_yaml::Error),
  #[error(transparent)]
  EnvVar(#[from] std::env::VarError),

  #[error(transparent)]
  IO(#[from] std::io::Error),

  // #[cfg(feature = "with-db")]
  #[error(transparent)]
  DB(#[from] sea_orm::DbErr),

  // #[error(transparent)]
  // ParseAddress(#[from] AddressError),
  #[error("{0}")]
  Hash(String),

  // API
  #[error("{0}")]
  Unauthorized(String),

  // API
  #[error("not found")]
  NotFound,

  #[error("{0}")]
  BadRequest(String),

  #[error("")]
  CustomError(StatusCode, ErrorDetail),

  #[error("internal server error")]
  InternalServerError,

  #[error(transparent)]
  InvalidHeaderValue(#[from] InvalidHeaderValue),

  #[error(transparent)]
  InvalidHeaderName(#[from] InvalidHeaderName),

  #[error(transparent)]
  InvalidMethod(#[from] InvalidMethod),

  #[error(transparent)]
  TaskJoinError(#[from] tokio::task::JoinError),

  // #[cfg(feature = "with-db")]
  // Model
  #[error(transparent)]
  Model(#[from] ModelError),

  // #[cfg(feature = "bg_redis")]
  // #[error(transparent)]
  // RedisPool(#[from] bb8::RunError<sidekiq::RedisError>),

  // #[cfg(feature = "bg_redis")]
  // #[error(transparent)]
  // Redis(#[from] sidekiq::redis_rs::RedisError),

  // #[cfg(feature = "bg_pg")]
  #[error(transparent)]
  Sqlx(#[from] sqlx::Error),

  // #[error(transparent)]
  // Storage(#[from] crate::storage::StorageError),
  #[error(transparent)]
  Cache(#[from] crate::cache::CacheError),

  #[error(transparent)]
  Any(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
  pub fn wrap(err: impl std::error::Error + Send + Sync + 'static) -> Self {
    Self::Any(Box::new(err)) //.bt()
  }

  pub fn msg(err: impl std::error::Error + Send + Sync + 'static) -> Self {
    Self::Message(err.to_string()) //.bt()
  }
  #[must_use]
  pub fn string(s: &str) -> Self {
    Self::Message(s.to_string())
  }
  #[must_use]
  pub fn bt(self) -> Self {
    let backtrace = std::backtrace::Backtrace::capture();
    match backtrace.status() {
      std::backtrace::BacktraceStatus::Disabled | std::backtrace::BacktraceStatus::Unsupported => {
        self
      }
      _ => Self::WithBacktrace {
        inner: Box::new(self),
        backtrace: Box::new(backtrace),
      },
    }
  }
}

/// Create an unauthorized error with a specified message.
///
/// This function is used to generate an `Error::Unauthorized` variant with a
/// custom message.
///
/// # Errors
///
/// returns unauthorized enum
///
/// # Example
///
/// ```rust
///
/// use axum_core::response::Response;
/// use tracing_subscriber::fmt::format;///
///
/// use pos_rust_local_backend::errors::unauthorized;
///
/// async fn login() -> pos_rust_local_backend::Result<Response> {
///     let valid = false;
///     if !valid {
///         return unauthorized("unauthorized access");
///     }
///     format::json()
/// }
/// ````
pub fn unauthorized<T: Into<String>, U>(msg: T) -> Result<U> {
  Err(Error::Unauthorized(msg.into()))
}

/// Return a bad request with a message
///
/// # Errors
///
/// This function will return an error result
pub fn bad_request<T: Into<String>, U>(msg: T) -> Result<U> {
  Err(Error::BadRequest(msg.into()))
}

/// return not found status code
///
/// # Errors
/// Currently this function doesn't return any error. this is for feature
/// functionality
pub fn not_found<T>() -> Result<T> {
  Err(Error::NotFound)
}
#[derive(Debug, Serialize)]
/// Structure representing details about an error.
pub struct ErrorDetail {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

impl ErrorDetail {
  /// Create a new `ErrorDetail` with the specified error and description.
  #[must_use]
  pub fn new<T: Into<String>>(error: T, description: T) -> Self {
    Self {
      error: Some(error.into()),
      description: Some(description.into()),
    }
  }

  /// Create an `ErrorDetail` with only an error reason and no description.
  #[must_use]
  pub fn with_reason<T: Into<String>>(error: T) -> Self {
    Self {
      error: Some(error.into()),
      description: None,
    }
  }
}

#[derive(Debug, FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
  fn into_response(self) -> Response {
    axum::Json(self.0).into_response()
  }
}

impl IntoResponse for Error {
  /// Convert an `Error` into an HTTP response.
  fn into_response(self) -> Response {
    match &self {
      Self::WithBacktrace {
        inner,
        backtrace: _,
      } => {
        tracing::error!(
        error.msg = %inner,
        error.details = ?inner,
        "controller_error"
        );
      }
      err => {
        tracing::error!(
        error.msg = %err,
        error.details = ?err,
        "controller_error"
        );
      }
    }

    let public_facing_error = match self {
      Self::NotFound => (
        StatusCode::NOT_FOUND,
        ErrorDetail::new("not_found", "Resource was not found"),
      ),
      Self::InternalServerError => (
        StatusCode::INTERNAL_SERVER_ERROR,
        ErrorDetail::new("internal_server_error", "Internal Server Error"),
      ),
      Self::Unauthorized(err) => {
        tracing::warn!(err);
        (
          StatusCode::UNAUTHORIZED,
          ErrorDetail::new(
            "unauthorized",
            "You do not have permission to access this resource",
          ),
        )
      }
      Self::CustomError(status_code, data) => (status_code, data),
      // Self::WithBacktrace { inner, backtrace } => {
      //     println!("\n{}", inner.to_string().red().underline());
      //     // backtrace::print_backtrace(&backtrace).unwrap();
      //     (
      //         StatusCode::BAD_REQUEST,
      //         ErrorDetail::with_reason("Bad Request"),
      //     )
      // }
      _ => (
        StatusCode::BAD_REQUEST,
        ErrorDetail::with_reason("Bad Request"),
      ),
    };

    (public_facing_error.0, Json(public_facing_error.1)).into_response()
  }
}

use sea_orm::sqlx;
use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct ModelValidation {
  pub code: String,
  pub message: Option<String>,
}

#[derive(thiserror::Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum ModelError {
  #[error("Entity already exists")]
  EntityAlreadyExists,

  #[error("Entity not found")]
  EntityNotFound,

  #[error("{errors:?}")]
  ModelValidation { errors: ModelValidation },

  // #[error("jwt error")]
  // Jwt(#[from] jsonwebtoken::errors::Error),
  #[error(transparent)]
  DbErr(#[from] sea_orm::DbErr),

  #[error(transparent)]
  Any(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[allow(clippy::module_name_repetitions)]
pub type ModelResult<T, E = ModelError> = std::result::Result<T, E>;
