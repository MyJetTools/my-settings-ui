use std::time::Duration;

pub fn unix_microseconds_to_string(src: i64) -> String {
    let d = std::time::UNIX_EPOCH + Duration::from_micros(src as u64);
    let dt = chrono::DateTime::<chrono::Utc>::from(d);
    dt.to_rfc3339()
}

/*

pub fn to_base_64(src: &str) -> String {
    base64::encode(src)
}

pub fn from_base_64(src: &str) -> String {
    let result = base64::decode(src).unwrap();
    String::from_utf8(result).unwrap()
}
 */
