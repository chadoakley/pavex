//! RFC 10008 QUERY method integration for Pavex.
//!
//! Scaffolds the components required to bring first-class HTTP QUERY
//! support to Pavex applications. Each function below has an RFC 10008
//! section reference indicating what it must satisfy.
//!
//! See `/rfc10008.md` at the repository root for the RFC excerpts.

use std::str::FromStr;

use crate::http::{HeaderMap, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};

/// Error surfaced by [`validate_content_type`].
#[derive(Debug, PartialEq, Eq)]
pub enum ContentTypeError {
    /// The `Content-Type` header is absent from the request.
    Missing,
    /// The `Content-Type` header is present but is inconsistent with the
    /// request content (per RFC 10008 §2).
    Mismatch,
}

/// Validate that a QUERY request carries a `Content-Type` header and that
/// its value is consistent with the request content, per RFC 10008 §2.
///
/// > Servers MUST fail the request if the Content-Type request field
/// > \[HTTP §8.3\] is missing or is inconsistent with the request content.
pub fn validate_content_type(
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(), ContentTypeError> {
    let Some(raw) = headers.get(CONTENT_TYPE) else {
        return Err(ContentTypeError::Missing);
    };
    let Ok(text) = raw.to_str() else {
        return Err(ContentTypeError::Mismatch);
    };
    let Ok(mime) = mime::Mime::from_str(text) else {
        return Err(ContentTypeError::Mismatch);
    };
    // A non-empty declared Content-Type paired with an empty body is
    // treated as inconsistent — the header claims content the request
    // doesn't actually carry.
    if body.is_empty() && mime.type_() != mime::APPLICATION_OCTET_STREAM.type_() {
        return Err(ContentTypeError::Mismatch);
    }
    Ok(())
}

/// Build an `Accept-Query` response header value advertising the query
/// media ranges the resource supports, per RFC 10008 §3.
///
/// The header value follows Structured Fields List syntax
/// \[STRUCTURED-FIELDS\] — media ranges as Tokens or Strings, comma-separated.
///
/// # Example (per RFC 10008 §3)
///
/// `Accept-Query: "application/jsonpath", application/sql;charset="UTF-8"`
pub fn accept_query_header(supported_media_ranges: &[&str]) -> HeaderValue {
    // Media types contain `/` which is disallowed in Structured Fields
    // Tokens, so we emit each entry as a quoted String — always safe.
    let joined = supported_media_ranges
        .iter()
        .map(|r| format!("\"{}\"", r.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(", ");
    HeaderValue::from_str(&joined).expect("Accept-Query header value must be ASCII")
}

/// Compute the cache key for a QUERY request, per RFC 10008 §2.7.
///
/// > The cache key for a QUERY request \[HTTP-CACHING §2\] MUST incorporate
/// > the request content \[HTTP-CACHING §6\] and related metadata
/// > \[HTTP §8\].
///
/// The returned bytes are opaque; only their equality matters for cache
/// lookup. Two requests with different content or content-type MUST produce
/// different keys.
pub fn cache_key_for(
    request_target: &str,
    content_type: &HeaderValue,
    body: &[u8],
) -> Vec<u8> {
    // Deterministic concatenation with 0x00 separators. Any change to
    // target, content-type, or body produces a different key.
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

/// Return the HTTP method a user agent should use when following a redirect
/// response to a QUERY request, per RFC 10008 §2.5.
///
/// > A response with either status codes 301, 302, 307 or 308 indicates
/// > that the user agent can accomplish its original QUERY request by
/// > sending a similar QUERY request to the new target URI. […] A response
/// > to QUERY with the status code 303 (See Other) indicates that the
/// > original query can be accomplished via a normal retrieval request on
/// > the URI referenced by the Location response field.
///
/// > Note that the exceptions for redirecting a POST as a GET request
/// > after a 301 or 302 response do NOT apply to QUERY requests.
pub fn redirect_method_for(status: StatusCode, original: &Method) -> Method {
    if status == StatusCode::SEE_OTHER {
        Method::GET
    } else {
        original.clone()
    }
}
