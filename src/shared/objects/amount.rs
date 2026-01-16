use crate::shared::{ids::AssetId, objects::errors::AmountTypeErrors, traits::DomainError};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Amount {
    value: f32,
    asset_id: AssetId,
}
impl Amount {
    pub(crate) fn new(value: f32, asset_id: &AssetId) -> Self {
        let id: AssetId = asset_id.clone();

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
    
}