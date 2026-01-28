use std::{collections::BTreeMap, rc::Rc};

use crate::{models::SecretHttpModel, states::MainState};

#[derive(Debug, Clone, Copy)]
pub enum OrderBy {
    Name,
    Updated,
}

pub struct SecretsListState {
    pub order_by: OrderBy,
    pub product_id: Rc<String>,
}

impl SecretsListState {
    pub fn new(ms_ra: &MainState) -> Self {
        let product_id = match crate::storage::last_used_product::get() {
            Some(product_id) => product_id,
            None => ms_ra.products.first().cloned().unwrap_or_default(),
        };

        Self {
            order_by: OrderBy::Name,
            product_id: Rc::new(product_id),
        }
    }

    pub fn sort<'a>(
        &self,
        secrets: impl Iterator<Item = &'a SecretHttpModel>,
    ) -> BTreeMap<String, &'a SecretHttpModel> {
        let mut result = BTreeMap::new();

        match self.order_by {
            OrderBy::Name => {
                for secret in secrets {
                    result.insert(secret.secret_id.clone(), secret);
                }
            }

            OrderBy::Updated => {
                for secret in secrets {
                    result.insert(
                        crate::utils::unix_microseconds_to_string(secret.updated).into_string(),
                        secret,
                    );
                }
            }
        }

        result
    }
}
