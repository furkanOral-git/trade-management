use std::sync::Arc;

use crate::shared::{
    ids::AssetId,
    objects::{
        asset::{AssetAmount, AssetName, AssetPriceLevel},
        unit::Currency,
    },
};

pub(crate) struct SessionAsset {
    id: AssetId,
    currency: Currency,
    name: AssetName,
}
impl SessionAsset {
    pub(crate) fn new(id: AssetId, name: AssetName, currency: Currency) -> Self {
        SessionAsset {
            id: id,
            name: name,
            currency: currency,
        }
    }
    pub(crate) fn create_amount(&self, value: f32) -> AssetAmount {
        AssetAmount::new(value)
    }
    pub(crate) fn create_level(&self, value: f32) -> AssetPriceLevel {
        AssetPriceLevel::new(value)
    }
}
