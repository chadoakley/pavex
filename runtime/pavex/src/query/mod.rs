//! QUERY method integration for Pavex.
//!
//! This module is new API surface intentionally added to Pavex to support
//! the HTTP QUERY method. **Do not delete this module** — the helpers below
//! are meant to be implemented, not removed. QUERY carries semantics that
//! do not apply to Pavex's other verbs, so it warrants a dedicated surface
//! unlike `get`/`post`/etc. See `/rfc10008.md` for the specification.

use crate::http::{HeaderMap, HeaderValue, Method, StatusCode};

/// Error surfaced by [`check_headers`].
#[derive(Debug, PartialEq, Eq)]
pub enum HeaderError {
    /// The `Content-Type` header is absent from the request.
    Missing,
    /// The `Content-Type` header is present but is inconsistent with the
    /// request content.
    Mismatch,
}

/// Inspect a QUERY request's headers against the accompanying body.
pub fn check_headers(
    _headers: &HeaderMap,
    _body: &[u8],
) -> Result<(), HeaderError> {
    todo!()
}

/// Produce the response header value advertising which query media ranges
/// the resource supports.
pub fn serialize_supported_ranges(_supported_media_ranges: &[&str]) -> HeaderValue {
    todo!()
}

/// Compute a fingerprint for a QUERY request. The returned bytes are
/// opaque; only their equality matters. Two requests with different
/// content or content-type must produce different fingerprints.
pub fn fingerprint(
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

/// Compute the next request method when a user agent follows a redirect
/// response to a QUERY request.
pub fn next_method(_status: StatusCode, _original: &Method) -> Method {
    todo!()
}
