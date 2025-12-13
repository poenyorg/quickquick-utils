use std::collections::HashMap;

use crate::types::{InnerMarketId, MarketName};

pub struct MarketMapping {
    pub markets_by_name: HashMap<MarketName, InnerMarketId>,
    pub markets_by_id: HashMap<InnerMarketId, MarketName>,
}

impl MarketMapping {
    pub fn map_to_id(&self, name: &MarketName) -> InnerMarketId {
        *self.markets_by_name.get(name).unwrap()
    }

    pub fn map_to_name(&self, id: &InnerMarketId) -> MarketName {
        self.markets_by_id.get(id).unwrap().clone()
    }

    pub fn is_market_name_supported(&self, name: &MarketName) -> bool {
        self.markets_by_name.contains_key(name)
    }

     pub fn is_market_id_supported(&self, id: &InnerMarketId) -> bool {
        self.markets_by_id.contains_key(id)
    }
}
