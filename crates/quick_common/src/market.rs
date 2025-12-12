use std::collections::HashMap;

use crate::types::{ExchangeId, InnerMarketId, MarketName};

pub struct MarketMapping {
    pub markets_by_name: HashMap<MarketName, InnerMarketId>,
    pub markets_by_id: HashMap<InnerMarketId, MarketName>,
}

impl MarketMapping {
    pub fn new(_exchange_id: ExchangeId) -> Self {
        Self {
            markets_by_name: HashMap::new(),
            markets_by_id: HashMap::new(),
        }
    }

    pub fn map_to_id(&self, name: &MarketName) -> InnerMarketId {
        *self.markets_by_name.get(name).unwrap()
    }

    pub fn map_to_name(&self, id: &InnerMarketId) -> MarketName {
        self.markets_by_id.get(id).unwrap().clone()
    }
}
