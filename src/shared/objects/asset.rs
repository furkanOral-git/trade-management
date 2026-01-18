#![allow(unused)]
use std::sync::Arc;

use crate::shared::{
    ids::AssetId,
    objects::{errors::AssetAmountTypeError, unit::Currency},
    traits::DomainError,
};

pub(crate) struct AssetName(String);

impl AssetName {
    pub fn new(name: impl Into<String>) -> Self {
        //uzunluk kontrolü, UPPER_CASE hale getirme, diğer kontroller.
        AssetName(name.into())
    }
    pub fn value(&self) -> &String {
        &self.0
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AssetPriceLevel {
    value: f32,
    currency: Arc<Currency>,
    id: Arc<AssetId>,
}

impl AssetPriceLevel {
    pub(crate) fn new(value: f32, currency: Arc<Currency>, asset_id: Arc<AssetId>) -> Self {
        
        AssetPriceLevel {
            value : value,
            currency: currency,
            id: asset_id,
        }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
    pub(crate) fn currency(&self) -> &Arc<Currency> {
        &self.currency
    }
    pub(crate) fn asset_id(&self)->&Arc<AssetId>{
        &self.id
    }
    pub(crate) fn calc_range(&self, level: &AssetPriceLevel) -> AssetPriceRange {
        let mut range = self.value - level.value();
        range = if range < 0.0 { -range } else { range };
        AssetPriceRange(range)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AssetPriceRange(f32);
impl AssetPriceRange {
    pub(crate) fn value(&self) -> &f32 {
        &self.0
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AssetAmount {
    value: f32,
    asset_id: Arc<AssetId>,
}
impl AssetAmount {
    pub(crate) fn new(value: f32, asset_id: Arc<AssetId>) -> Self {
        
        AssetAmount {
            value: value,
            asset_id: asset_id,
        }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
    pub(crate) fn asset_id(&self) -> &Arc<AssetId> {
        &self.asset_id
    }
}

impl AssetAmount {
    pub(crate) fn add_amount(self, amount: &AssetAmount) -> AssetAmount {
        if self.asset_id == amount.asset_id {
            return AssetAmount {
                value: self.value + amount.value,
                asset_id: self.asset_id,
            };
        }
        panic!(
            "{}",
            AssetAmountTypeError::DifferentAmountOfAssetsCanNotBeAdd.message()
        )
    }
    pub(crate) fn remove_amount(self, amount: &AssetAmount) -> AssetAmount {
        if self.asset_id == amount.asset_id {
            return AssetAmount {
                value: self.value - amount.value,
                asset_id: self.asset_id,
            };
        }
        panic!(
            "{}",
            AssetAmountTypeError::DifferentAmountOfAssetsCanNotBeRemove.message()
        )
    }
}
