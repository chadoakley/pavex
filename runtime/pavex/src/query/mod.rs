//! RFC 10008 QUERY method integration for Pavex.
//!
//! Scaffolds the components required to bring first-class HTTP QUERY
//! support to Pavex applications. Each function below has an RFC 10008
//! section reference indicating what it must satisfy.
//!
//! See `/rfc10008.md` at the repository root for the RFC excerpts.

use crate::http::{HeaderMap, HeaderValue, Method, StatusCode};

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
    _headers: &HeaderMap,
    _body: &[u8],
) -> Result<(), ContentTypeError> {
    todo!("RFC 10008 §2: check Content-Type presence + consistency with body")
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
pub fn accept_query_header(_supported_media_ranges: &[&str]) -> HeaderValue {
    todo!("RFC 10008 §3: build Accept-Query header from supported media ranges")
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
    _request_target: &str,
    _content_type: &HeaderValue,
    _body: &[u8],
) -> Vec<u8> {
    todo!("RFC 10008 §2.7: compose cache key from target + content-type + body")
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
pub fn redirect_method_for(_status: StatusCode, _original: &Method) -> Method {
    todo!("RFC 10008 §2.5: 303 downgrades to GET; 301/302/307/308 preserve method")
}
