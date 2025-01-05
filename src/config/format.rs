//! This module contains utility functions for generating HTTP responses that
//! are commonly used in web applications. These functions simplify the process
//! of creating responses with various data types.
//!
//! # Example:
//!
//! This example illustrates how to construct a JSON-formatted response using a
//! Rust struct.
//!
//! ```rust
//! use axum_core::response::Response;
//! use serde::Serialize;//!
//!
//! use pos_rust_local_backend::config::format;
//!
//! #[derive(Serialize)]
//! pub struct Health {
//!     pub ok: bool,
//! }
//!
//! async fn ping() -> pos_rust_local_backend::Result<Response> {
//!    format::json(Health { ok: true })
//! }
//! ```
use crate::Result;
use axum::{
  response::{Html, IntoResponse, Redirect, Response},
  Json,
};
use serde::Serialize;
use serde_json::json;

/// Returns an empty response.
///
/// # Example:
///
/// This example illustrates how to return an empty response.
/// ```rust
/// use axum_core::response::Response;
///
///
/// use pos_rust_local_backend::config::format;
///
/// async fn endpoint() -> pos_rust_local_backend::Result<Response> {
///    format::empty()
/// }
/// ```
///
/// # Errors
///
/// Currently this function doesn't return any error. this is for feature
/// functionality
pub fn empty() -> Result<Response> {
  Ok(().into_response())
}

/// Returns a response containing the provided text.
///
/// # Example:
///
/// This example illustrates how to return an text response.
/// ```rust
///
///
/// use axum_core::response::Response;
/// use pos_rust_local_backend::config::format;
///
///  async fn endpoint() -> pos_rust_local_backend::Result<Response> {
///    format::text("MESSAGE-RESPONSE")
/// }
/// ```
///
/// # Errors
///
/// Currently this function doesn't return any error. this is for feature
/// functionality
pub fn text(t: &str) -> Result<Response> {
  Ok(t.to_string().into_response())
}

/// Returns a JSON response containing the provided data.
///
/// # Example:
///
/// This example illustrates how to construct a JSON-formatted response using a
/// Rust struct.
///
/// ```rust
///
/// use axum_core::response::Response;
/// use serde::Serialize;///
///
/// use pos_rust_local_backend::config::format;
///
/// #[derive(Serialize)]
/// pub struct Health {
///     pub ok: bool,
/// }
///
/// async fn endpoint() -> pos_rust_local_backend::Result<Response> {
///    format::json(Health { ok: true })
/// }
/// ```
///
/// # Errors
///
/// Currently this function doesn't return any error. this is for feature
/// functionality
pub fn json<T: Serialize>(t: T) -> Result<Response> {
  Ok(Json(t).into_response())
}

/// Respond with empty json (`{}`)
///
/// # Errors
///
/// This function will return an error if serde fails
pub fn empty_json() -> Result<Response> {
  json(json!({}))
}

/// Returns an HTML response
///
/// # Example:
///
/// ```rust
///
///
/// use axum_core::response::Response;
/// use pos_rust_local_backend::config::format;
///
///  async fn endpoint() -> pos_rust_local_backend::Result<Response> {
///    format::html("hello, world")
/// }
/// ```
///
/// # Errors
///
/// Currently this function doesn't return any error. this is for feature
/// functionality
pub fn html(content: &str) -> Result<Response> {
  Ok(Html(content.to_string()).into_response())
}

/// Returns an redirect response
///
/// # Example:
///
/// ```rust
///
///
/// use axum_core::response::Response;
/// use pos_rust_local_backend::config::format;
///
///  async fn login() -> pos_rust_local_backend::Result<Response> {
///    format::redirect("/dashboard")
/// }
/// ```
///
/// # Errors
///
/// Currently this function doesn't return any error. this is for feature
/// functionality
pub fn redirect(to: &str) -> Result<Response> {
  Ok(Redirect::to(to).into_response())
}
