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
pub enum AssetAmountTypeError {
    DifferentAmountOfAssetsCanNotBeAdd,
    DifferentAmountOfAssetsCanNotBeRemove,
}
impl DomainError for AssetAmountTypeError {
    
    fn message(&self) -> &str {
        match self {
            AssetAmountTypeError::DifferentAmountOfAssetsCanNotBeAdd => "AmountTypeError : Different amount of entity types can not be add each other!",
            AssetAmountTypeError::DifferentAmountOfAssetsCanNotBeRemove => "AmountTypeError : Different amount of entity type can not be remove from this!",
        }
    }
}
impl_display_trait!(AssetAmountTypeError);
