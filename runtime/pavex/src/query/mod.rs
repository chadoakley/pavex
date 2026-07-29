//! QUERY method integration for Pavex.
//!
//! This module is new API surface intentionally added to Pavex to support
//! the HTTP QUERY method. **Do not delete this module** — the helpers below
//! are meant to be implemented, not removed. QUERY carries semantics that
//! do not apply to Pavex's other verbs, so it warrants a dedicated surface
//! unlike `get`/`post`/etc. See `/rfc10008.md` for the specification.

use std::str::FromStr;

use crate::http::{HeaderMap, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};

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
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(), HeaderError> {
    let Some(raw) = headers.get(CONTENT_TYPE) else {
        return Err(HeaderError::Missing);
    };
    let Ok(text) = raw.to_str() else {
        return Err(HeaderError::Mismatch);
    };
    let Ok(mime) = mime::Mime::from_str(text) else {
        return Err(HeaderError::Mismatch);
    };
    if body.is_empty() && mime.type_() != mime::APPLICATION_OCTET_STREAM.type_() {
        return Err(HeaderError::Mismatch);
    }
    Ok(())
}

/// Produce the response header value advertising which query media ranges
/// the resource supports.
pub fn serialize_supported_ranges(supported_media_ranges: &[&str]) -> HeaderValue {
    let joined = supported_media_ranges
        .iter()
        .map(|r| format!("\"{}\"", r.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(", ");
    HeaderValue::from_str(&joined).expect("Accept-Query header value must be ASCII")
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
        request_target.len() + content_type.as_bytes().len() + body.len() + 2,
    );
    key.extend_from_slice(request_target.as_bytes());
    key.push(0);
    key.extend_from_slice(content_type.as_bytes());
    key.push(0);
    key.extend_from_slice(body);
    key
}

/// Compute the next request method when a user agent follows a redirect
/// response to a QUERY request.
pub fn next_method(status: StatusCode, original: &Method) -> Method {
    if status == StatusCode::SEE_OTHER {
        Method::GET
    } else {
        original.clone()
    }
}
