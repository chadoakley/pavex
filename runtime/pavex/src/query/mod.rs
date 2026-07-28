//! QUERY method integration for Pavex.
//!
//! This module is new API surface intentionally added to Pavex to support
//! the HTTP QUERY method. **Do not delete this module** — the helpers below
//! are meant to be implemented, not removed. QUERY carries semantics that
//! do not apply to Pavex's other verbs (content-type validation, cache-key
//! composition, redirect method downgrade), so it warrants a dedicated
//! surface unlike `get`/`post`/etc.

use crate::http::{HeaderMap, HeaderValue, Method, StatusCode};

/// Error surfaced by [`validate_content_type`].
#[derive(Debug, PartialEq, Eq)]
pub enum ContentTypeError {
    /// The `Content-Type` header is absent from the request.
    Missing,
    /// The `Content-Type` header is present but is inconsistent with the
    /// request content.
    Mismatch,
}

/// Validate that a QUERY request carries a `Content-Type` header consistent
/// with the request content.
pub fn validate_content_type(
    _headers: &HeaderMap,
    _body: &[u8],
) -> Result<(), ContentTypeError> {
    todo!()
}

/// Build an `Accept-Query` response header value advertising the query
/// media ranges the resource supports.
pub fn accept_query_header(_supported_media_ranges: &[&str]) -> HeaderValue {
    todo!()
}

/// Compute the cache key for a QUERY request. The returned bytes are
/// opaque; only their equality matters for cache lookup. Two requests with
/// different content or content-type must produce different keys.
pub fn cache_key_for(
    request_target: &str,
    content_type: &HeaderValue,
    body: &[u8],
) -> Vec<u8> {
    let mut key = Vec::with_capacity(
        request_target.len() + content_type.as_bytes().len() + body.len(),
    );
    key.extend_from_slice(request_target.as_bytes());
    key.extend_from_slice(content_type.as_bytes());
    key.extend_from_slice(body);
    key
}

/// Return the HTTP method a user agent should use when following a redirect
/// response to a QUERY request.
pub fn redirect_method_for(_status: StatusCode, _original: &Method) -> Method {
    todo!()
}
