use std::sync::Arc;

use crate::shared::{objects::errors::UnitTypeErrors, traits::DomainError};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Currency {
    USD,
    USDT,
    USDC,
    TL,
}
impl Currency {
    pub(crate) fn stringfy(&self) -> String {
        match self {
            Currency::USD => "$".into(),
            Currency::USDT => "₮".into(),
            Currency::USDC => "USDC".into(),
            Currency::TL => "₺".into(),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    value: f32,
    currency: Arc<Currency>,
}
impl Unit {
    pub(crate) fn new(currency: Arc<Currency>, value: f32) -> Self {
        
        Unit {
            value,
            currency: currency,
        }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
    pub(crate) fn currency(&self) -> &Currency {
        &self.currency
    }
}
impl Unit {
    pub(crate) fn add_unit(self, unit: &Unit) -> Unit {
        
        if unit.currency == self.currency {
            return Unit {
                value: self.value + unit.value,
                currency: self.currency,
            };
        }

        panic!(
            "{}",
            UnitTypeErrors::DifferentUnitTypesCanNotBeAdd.message()
        )
    }
    pub(crate) fn remove_unit(self, unit: &Unit) -> Unit {
        if unit.currency == self.currency {
            return Unit {
                value: self.value - unit.value,
                currency: self.currency,
            };
        }
        panic!(
            "{}",
            UnitTypeErrors::DifferentUnitTypesCanNotBeRemove.message()
        )
    }
    pub(crate) fn as_string(&self) -> String {
        format!("{} {}", self.value, self.currency.stringfy())
    }
}