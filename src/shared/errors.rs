use crate::{impl_display_trait, shared::traits::DomainError};

#[derive(Debug)]
pub enum UnitTypeErrors {
    DifferentUnitTypesCanNotBeAdd,
    DifferentUnitTypesCanNotBeRemove,
}
impl DomainError for UnitTypeErrors {
    
    fn message(&self) -> &str {
        match self {
            UnitTypeErrors::DifferentUnitTypesCanNotBeAdd => {
                "UnitTypeError : Different unit types can not be add to each other!"
            }
            UnitTypeErrors::DifferentUnitTypesCanNotBeRemove => {
                "UnitTypeError : Different unit type can not be remove!"
            }
        }
    }
}
impl_display_trait!(UnitTypeErrors);
#[derive(Debug)]
pub enum AmountTypeErrors {
    DifferentAmountOfEntityCanNotBeAdd,
    DifferentAmountOfEntityCanNotBeRemove,
}
impl DomainError for AmountTypeErrors {
    
    fn message(&self) -> &str {
        match self {
            AmountTypeErrors::DifferentAmountOfEntityCanNotBeAdd => "AmountTypeError : Different amount of entity types can not be add each other!",
            AmountTypeErrors::DifferentAmountOfEntityCanNotBeRemove => "AmountTypeError : Different amount of entity type can not be remove from this!",
        }
    }
}
impl_display_trait!(AmountTypeErrors);
