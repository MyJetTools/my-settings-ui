use std::time::Duration;

pub struct DateTimeRfc3339(String);

impl DateTimeRfc3339 {
    pub fn without_microseconds(&self) -> &str {
        &self.0[..19]
    }

    /*
    pub fn into_string(self) -> String {
        self.0
    }
     */
}

pub fn unix_microseconds_to_string(src: i64) -> DateTimeRfc3339 {
    let d = std::time::UNIX_EPOCH + Duration::from_micros(src as u64);
    let dt = chrono::DateTime::<chrono::Utc>::from(d);
    DateTimeRfc3339(dt.to_rfc3339())
}
/*

pub fn extract_domain_name(src: &str) -> &str {
    let mut found_pos = 0;
    let mut found_pos_prev = 0;

    let src_bytes = src.as_bytes();
    for i in 0..src_bytes.len() {
        if src_bytes[i] == b'.' {
            found_pos_prev = found_pos;
            found_pos = i;
        }
    }

    if src_bytes[found_pos_prev] == b'.' {
        return &src[found_pos_prev + 1..];
    }

    &src[found_pos_prev..]
}


pub fn to_base_64(src: &str) -> String {
    base64::encode(src)
}

pub fn from_base_64(src: &str) -> String {
    let result = base64::decode(src).unwrap();
    String::from_utf8(result).unwrap()
}
 */
