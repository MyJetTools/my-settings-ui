use super::EditDomainProductProps;

pub struct EditDomainProductState {
    add: bool,
    pub product_name_init: String,
    pub product_name: String,
    pub is_cloud_flare_proxy_init: bool,
    pub is_cloud_flare_proxy: bool,
    pub internal_domain_name_init: String,
    pub internal_domain_name: String,
}

impl EditDomainProductState {
    pub fn new(src: &EditDomainProductProps) -> Self {
        Self {
            add: src.add,
            product_name_init: src.name.to_string(),
            product_name: src.name.to_string(),
            is_cloud_flare_proxy: src.is_cloud_flare_proxy_pass,
            is_cloud_flare_proxy_init: src.is_cloud_flare_proxy_pass,
            internal_domain_name_init: src.internal_domain_name.to_string(),
            internal_domain_name: src.internal_domain_name.to_string(),
        }
    }

    pub fn get_product_name(&self) -> &str {
        self.product_name.as_str()
    }

    pub fn get_internal_domain_name(&self) -> &str {
        self.internal_domain_name.as_str()
    }

    pub fn set_product_name(&mut self, product_name: &str) {
        self.product_name = product_name.to_string();
    }

    pub fn set_internal_domain_name(&mut self, value: &str) {
        self.internal_domain_name = value.to_string();
    }

    pub fn can_be_saved(&self) -> bool {
        if self.product_name.len() == 0 {
            return false;
        }

        if self.internal_domain_name.len() == 0 {
            return false;
        }

        if self.add {
            self.product_name_init != self.product_name
                || self.internal_domain_name_init != self.internal_domain_name
        } else {
            self.product_name_init != self.product_name
                || self.internal_domain_name_init != self.internal_domain_name
                || self.is_cloud_flare_proxy_init != self.is_cloud_flare_proxy
        }
    }
}
