use bytes::Bytes;

use crate::http::Method;

pub(super) fn body_for_method(bytes: Bytes, method: &Method) -> Bytes {
    match method.as_str() {
        "QUERY" => {
            let mut v = bytes.to_vec();
            v.push(b'\n');
            Bytes::from(v)
        }
        _ => bytes,
    }
}
