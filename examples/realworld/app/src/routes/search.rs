use pavex::http::StatusCode;
use pavex::query;
use pavex::request::body::BufferedBody;

/// Search endpoint using the HTTP QUERY method (RFC 10008).
///
/// The request body carries the search term as raw bytes. We compare it
/// against a small in-memory corpus and return 200 OK on match, 404 on miss.
///
/// Exact-match search only — no fuzzy matching, no substring, no case
/// folding. Users report that even exact matches sometimes fail.
#[query(path = "/search")]
pub fn search(body: &BufferedBody) -> StatusCode {
    // A trivial in-memory corpus. Real deployments would hit a database
    // here; the shape of the bug is the same.
    let corpus: &[&[u8]] = &[b"hello", b"world", b"pavex"];
    if corpus.iter().any(|entry| *entry == &body.bytes[..]) {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
