use pavex::http::{HeaderValue, header::CONTENT_TYPE};
use pavex::request::RequestHead;
use pavex::request::body::BufferedBody;
use pavex::request::hasher;
use pavex::request_scoped;

/// A per-request identifier used to attribute traces, cache lookups, and
/// audit trails to a single logical search operation.
pub struct SearchKey(pub Vec<u8>);

#[request_scoped]
pub fn search_key(head: &RequestHead, body: &BufferedBody) -> SearchKey {
    let empty = HeaderValue::from_static("");
    let ct = head.headers.get(CONTENT_TYPE).unwrap_or(&empty);
    SearchKey(hasher::compute(&head.target.to_string(), ct, &body.bytes))
}
