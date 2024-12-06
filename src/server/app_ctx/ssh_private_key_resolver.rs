use std::sync::{atomic::AtomicBool, Arc};

use tokio::sync::Mutex;

use crate::server::settings::AppSettingsReader;

pub struct SshPrivateKeyResolver {
    pub settings_reader: Arc<AppSettingsReader>,
    pub pass_phrase: Mutex<Option<String>>,
    pub has_pass_key: AtomicBool,
}

impl SshPrivateKeyResolver {
    pub fn new(settings_reader: Arc<AppSettingsReader>) -> Self {
        Self {
            settings_reader,
            pass_phrase: Mutex::new(None),
            has_pass_key: AtomicBool::new(false),
        }
    }

    pub async fn set_pass_phrase(&self, pass_phrase: String) {
        let mut write_access = self.pass_phrase.lock().await;
        *write_access = Some(pass_phrase);
        self.has_pass_key
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn get_pass_phrase(&self) -> Option<String> {
        let read_access = self.pass_phrase.lock().await;
        read_access.clone()
    }

    pub fn has_pass_phrase(&self) -> bool {
        self.has_pass_key.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[async_trait::async_trait]
impl my_ssh::ssh_settings::SshSecurityCredentialsResolver for SshPrivateKeyResolver {
    async fn resolve_ssh_private_key(
        &self,
        ssh_line: &str,
    ) -> Option<my_ssh::ssh_settings::SshPrivateKey> {
        let settings = self.settings_reader.get_settings().await;

        let ssh_private_keys = settings.ssh_private_keys.as_ref()?;

        if let Some(ssh_credentials) = ssh_private_keys.get(ssh_line) {
            let mut pass_phrase = ssh_credentials.cert_pass_phrase.clone();

            if pass_phrase.is_none() {
                pass_phrase = self.get_pass_phrase().await;
            }

            return my_ssh::ssh_settings::SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase,
            }
            .into();
        }

        if let Some(ssh_credentials) = ssh_private_keys.get("*") {
            let mut pass_phrase = ssh_credentials.cert_pass_phrase.clone();

            if pass_phrase.is_none() {
                pass_phrase = self.get_pass_phrase().await;
            }

            return my_ssh::ssh_settings::SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase,
            }
            .into();
        }

        None
    }

    async fn resolve_ssh_password(&self, _ssh_line: &str) -> Option<String> {
        return None;
    }
}
