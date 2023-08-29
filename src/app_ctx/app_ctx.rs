use std::sync::Arc;

use tokio::sync::RwLock;

use crate::settings::SettingsReader;

pub struct AppCtxInner {
    settings_reader: Arc<SettingsReader>,
}

pub struct AppCtx {
    inner: RwLock<Option<Arc<AppCtxInner>>>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }

    pub async fn inject_settings(&self, settings_reader: Arc<SettingsReader>) {
        let mut write_access = self.inner.write().await;

        write_access.replace(Arc::new(AppCtxInner {
            settings_reader: settings_reader,
        }));
    }

    pub async fn get_settings_reader(&self) -> Arc<SettingsReader> {
        let read_access = self.inner.read().await;
        read_access.as_ref().unwrap().settings_reader.clone()
    }
}
