const PRODUCT_ID_LOCAL_STORAGE_KEY: &str = "product_id";
pub fn get() -> Option<String> {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().get(PRODUCT_ID_LOCAL_STORAGE_KEY)
}

pub fn save(value: &str) {
    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .set(PRODUCT_ID_LOCAL_STORAGE_KEY, value);
}
