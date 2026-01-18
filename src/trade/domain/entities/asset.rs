use std::sync::Arc;

use crate::shared::{
    ids::AssetId,
    objects::{
        asset::{AssetAmount, AssetName, AssetPriceLevel},
        unit::Currency,
    },
};

pub(crate) struct SessionAsset {
    id: Arc<AssetId>,
    currency: Arc<Currency>,
    name: AssetName,
}
impl SessionAsset {
    pub(crate) fn new(id: AssetId, name: AssetName, currency: Currency) -> Self {
        let arc_id = Arc::new(id);
        let arc_currency = Arc::new(currency);
        SessionAsset {
            id: arc_id,
            name: name,
            currency: arc_currency,
        }
    }
    pub(crate) fn create_amount(&self, value: f32) -> AssetAmount {
        AssetAmount::new(value, self.id.clone())
    }
    pub(crate) fn create_level(&self, value: f32) -> AssetPriceLevel {
        AssetPriceLevel::new(value, self.currency.clone(), self.id.clone())
    }
}
