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
}
impl Unit {
    pub(crate) fn new(value: f32) -> Self {
        Unit { value }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
}
impl Unit {
    
    pub(crate) fn as_string(&self) -> String {
        format!("{}", self.value)
    }
}
