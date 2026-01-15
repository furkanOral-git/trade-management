

use crate::shared::{objects::Unit};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum UnitType {
    USD,
    USDT,
    USDC,
    TL,
}
impl UnitType {
    pub(crate) fn stringfy(&self) -> String {
        match self {
            UnitType::USD => "$".into(),
            UnitType::USDT => "₮".into(),
            UnitType::USDC => "USDC".into(),
            UnitType::TL => "₺".into(),
        }
    }
    pub(crate) fn create_unit(&self, value: f32) -> Unit {
        Unit::new(self, value)
    }
    
}
