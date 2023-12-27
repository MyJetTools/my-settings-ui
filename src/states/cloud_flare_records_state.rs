use std::collections::HashMap;

use crate::views::CfRecordRestApiModel;

pub struct CloudFlareRecordsState {
    values: Option<HashMap<String, CfRecordRestApiModel>>,
}

impl CloudFlareRecordsState {
    pub fn new() -> Self {
        Self { values: None }
    }

    pub fn get_value(&self) -> Option<&HashMap<String, CfRecordRestApiModel>> {
        self.values.as_ref()
    }

    pub fn set_value(&mut self, value: Vec<CfRecordRestApiModel>) {
        let value = value
            .into_iter()
            .map(|itm| (itm.name.to_lowercase(), itm))
            .collect::<HashMap<_, _>>();

        self.values = Some(value);
    }

    pub fn reset_value(&mut self) {
        self.values = None;
    }
}
