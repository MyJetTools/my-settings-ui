use std::collections::HashMap;

const PRODUCT_ID_LOCAL_STORAGE_KEY: &str = "product_id";
pub fn get(env_id: &str) -> Option<String> {
    let mut result = get_as_hashmap()?;
    result.remove(env_id)
}

pub fn save(env_id: &str, value: &str) {
    let mut result = get_as_hashmap().unwrap_or_default();

    result.insert(env_id.to_string(), value.to_string());

    let result = serde_json::to_string(&result).unwrap();

    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .set(PRODUCT_ID_LOCAL_STORAGE_KEY, result.as_str());
}

fn get_as_hashmap() -> Option<HashMap<String, String>> {
    let result = dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(PRODUCT_ID_LOCAL_STORAGE_KEY)?;

    serde_json::from_str(result.as_str()).ok()?
}
