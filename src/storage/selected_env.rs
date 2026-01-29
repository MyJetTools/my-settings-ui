const ENV_LOCAL_STORAGE_KEY: &str = "env";
pub fn get() -> String {
    dioxus_utils::js::GlobalAppSettings::get_local_storage()
        .get(ENV_LOCAL_STORAGE_KEY)
        .unwrap_or_default()
}

pub fn save(value: &str) {
    dioxus_utils::js::GlobalAppSettings::get_local_storage().set(ENV_LOCAL_STORAGE_KEY, value);
}
