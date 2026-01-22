#![allow(unused)]

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
}

impl AssetPriceLevel {
    pub(crate) fn new(value: f32) -> Self {
        AssetPriceLevel { value: value }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
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
}
impl AssetAmount {
    pub(crate) fn new(value: f32) -> Self {
        AssetAmount { value: value }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
}


