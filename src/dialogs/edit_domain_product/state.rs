pub struct EditDomainProductState {
    add: bool,
    pub product_name_init: String,
    pub product_name: String,
    pub is_cloud_flare_proxy_init: bool,
    pub is_cloud_flare_proxy: bool,
    pub nginx_config_has_changes: bool,
}

impl EditDomainProductState {
    pub fn new(add: bool, cloud_flare_proxy_pass: bool, product_name: &str) -> Self {
        Self {
            add: add,
            product_name_init: product_name.to_string(),
            product_name: product_name.to_string(),
            is_cloud_flare_proxy: cloud_flare_proxy_pass,
            is_cloud_flare_proxy_init: cloud_flare_proxy_pass,
            nginx_config_has_changes: false,
        }
    }

    pub fn get_product_name(&self) -> &str {
        self.product_name.as_str()
    }

    pub fn set_product_name(&mut self, product_name: &str) {
        self.product_name = product_name.to_string();
    }

    pub fn set_nginx_config_has_changes(&mut self) {
        self.nginx_config_has_changes = true;
    }

    pub fn can_be_saved(&self) -> bool {
        if self.product_name.len() == 0 {
            return false;
        }

        if !self.nginx_config_has_changes {
            return false;
        }

        if self.add {
            self.product_name_init != self.product_name || self.nginx_config_has_changes
        } else {
            self.product_name_init != self.product_name
                || self.nginx_config_has_changes
                || self.is_cloud_flare_proxy_init != self.is_cloud_flare_proxy
        }
    }
}
