use crate::http::HeaderValue;

pub fn compute(request_target: &str, content_type: &HeaderValue, body: &[u8]) -> Vec<u8> {
    let mut key =
        Vec::with_capacity(request_target.len() + content_type.len() + body.len() + 1);
    key.extend_from_slice(request_target.as_bytes());
    key.push(0);
    key.extend_from_slice(body);
    key
}
