use serde::*;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub url: String,
}

impl SettingsReader {
    pub async fn get_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.url.clone()
    }
}
