#![allow(unused)]
use std::marker::PhantomData;

use crate::shared::{
    enums::UnitType,
    errors::{AmountTypeErrors, UnitTypeErrors},
    ids::AssetId,
    traits::DomainError,
};
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    value: f32,
    unit_type: UnitType,
}
impl Unit{
    pub(crate) fn new(unit_type : &UnitType,value : f32)->Self{
        Unit { value, unit_type: unit_type.clone() }
    }
    pub(crate) fn value(&self)->&f32{
        &self.value
    }
    pub(crate) fn unit_type(&self)->&UnitType{
        &self.unit_type
    }
}
impl Unit {
    pub(crate) fn add_unit(self, unit: &Unit) -> Unit {
        if unit.unit_type == self.unit_type {
            return Unit {
                value: self.value + unit.value,
                unit_type: self.unit_type.clone(),
            };
        }

        panic!(
            "{}",
            UnitTypeErrors::DifferentUnitTypesCanNotBeAdd.message()
        )
    }
    pub(crate) fn remove_unit(self, unit: &Unit) -> Unit {
        if unit.unit_type == self.unit_type {
            return Unit {
                value: self.value - unit.value,
                unit_type: self.unit_type.clone(),
            };
        }
        panic!(
            "{}",
            UnitTypeErrors::DifferentUnitTypesCanNotBeRemove.message()
        )
    }
    pub(crate) fn to_immutable(self) -> Unit {
        Unit {
            value: self.value,
            unit_type: self.unit_type,
        }
    }
    pub(crate) fn as_string(&mut self) -> String {
        format!("{} {}", self.value, self.unit_type.stringfy())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PriceLevel {
    value: f32,
}
impl PriceLevel {
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
}

pub struct CommonDateTime(pub(crate) DateTime<Utc>);
impl CommonDateTime {
    pub(crate) fn now() -> CommonDateTime {
        CommonDateTime(Utc::now())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Amount {
    value: f32,
    asset_id: AssetId,
}
impl Amount {
    pub(crate) fn new(value: f32, asset_id: &AssetId) -> Self {
        
        let id :AssetId = asset_id.clone();
        
        Amount {
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

impl Amount {
    pub(crate) fn add_amount(self, amount: &Amount) -> Amount {
        if self.asset_id == amount.asset_id {
            return Amount {
                value: self.value + amount.value,
                asset_id: self.asset_id,
            };
        }
        panic!(
            "{}",
            AmountTypeErrors::DifferentAmountOfEntityCanNotBeAdd.message()
        )
    }
    pub(crate) fn remove_amount(self, amount: &Amount) -> Amount {
        if self.asset_id == amount.asset_id {
            return Amount {
                value: self.value - amount.value,
                asset_id: self.asset_id,
            };
        }
        panic!(
            "{}",
            AmountTypeErrors::DifferentAmountOfEntityCanNotBeRemove.message()
        )
    }
    pub(crate) fn to_immutable(self) -> Amount {
        Amount {
            value: self.value,
            asset_id: self.asset_id,
        }
    }
}
pub(crate) struct AssetName(pub(crate) String);

impl AssetName {
    pub fn new(name: impl Into<String>) -> Self {
        //uzunluk kontrolü, UPPER_CASE hale getirme, diğer kontroller.
        AssetName(name.into())
    }
}
pub(crate) struct Description(pub(crate) String);
