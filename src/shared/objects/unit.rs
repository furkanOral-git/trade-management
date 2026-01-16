use crate::shared::{objects::errors::UnitTypeErrors, traits::DomainError};

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
#[derive(Debug, PartialEq, Clone)]
pub struct Unit {
    value: f32,
    unit_type: UnitType,
}
impl Unit {
    pub(crate) fn new(unit_type: &UnitType, value: f32) -> Self {
        Unit {
            value,
            unit_type: unit_type.clone(),
        }
    }
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
    pub(crate) fn unit_type(&self) -> &UnitType {
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
    pub(crate) fn as_string(&mut self) -> String {
        format!("{} {}", self.value, self.unit_type.stringfy())
    }
}