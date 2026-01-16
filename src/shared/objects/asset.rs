use crate::shared::{
    ids::AssetId,
    objects::{errors::AssetAmountTypeError, unit::{Unit, UnitType}}, traits::DomainError,
};

pub(crate) struct AssetName(pub(crate) String);

impl AssetName {
    pub fn new(name: impl Into<String>) -> Self {
        //uzunluk kontrolü, UPPER_CASE hale getirme, diğer kontroller.
        AssetName(name.into())
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AssetPriceLevel {
    value: f32,
    value_type_as_unit: UnitType,
}
impl AssetPriceLevel {
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
    pub(crate) fn calc_range(&self, level: &AssetPriceLevel) -> f32 {
        let mut range = self.value - level.value();
        range = if range < 0.0 { -range } else { range };
        range
    }
    pub(crate) fn calc_size_as_unit(&self, amount: &AssetAmount) -> Unit {
        let size_as_f32 = self.value * amount.value();
        self.value_type_as_unit.create_unit(size_as_f32)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AssetAmount {
    value: f32,
    asset_id: AssetId,
}
impl AssetAmount {
    pub(crate) fn new(value: f32, asset_id: &AssetId) -> Self {
        let id: AssetId = asset_id.clone();

        AssetAmount {
            value: value,
            asset_id: id,
        }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
    pub(crate) fn asset_id(&self) -> &AssetId {
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
