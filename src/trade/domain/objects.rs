#![allow(unused)]
use std::marker::PhantomData;

use crate::shared::{
    enums::UnitType,
    ids::AssetId,
    objects::{Amount, CommonDateTime, PriceLevel, Unit},
    traits::ValueObject,
};
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PositionType {
    Long,
    Short,
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TradeType {
    Buy,
    Sell,
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TradeAction {
    PartialClose,
    FullClose,
    AddSize,
    Initial,
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PositionStatus {
    status_level: PriceLevel,
    status_amount: Amount,
    status_stop: PriceLevel,
    status_risk: Unit,
    status_size: PositionSize,
}
impl PositionStatus{
    pub(crate) fn get_status_level(&self) ->&PriceLevel{
        &self.status_level
    }
    pub(crate) fn get_status_amount(&self)->&Amount{
        &self.status_amount
    } 
    pub(crate) fn get_status_stop(&self)->&PriceLevel{
        &self.status_stop
    }
    pub(crate) fn get_status_risk(&self)->&Unit{
        &self.status_risk
    }
    pub(crate) fn get_status_size(&self)->&PositionSize{
        &self.status_size
    }
}
impl PositionStatus {
    pub(crate) fn new(
        status_level: PriceLevel,
        status_stop_level: PriceLevel,
        status_risk: Unit,
        asset_id: AssetId,
    ) -> Self {
        let mut entry_stop_distance = status_level.value() - status_stop_level.value();
        if entry_stop_distance < 0.0 {
            entry_stop_distance *= -1.0;
        }
        // amount(belirsiz olan X dedik) * steps(belli olan) = risk_as_unit
        //risk_as_unit / steps = amount
        let amount_as_f32 = *status_risk.value() / entry_stop_distance;
        let amount = Amount::new(amount_as_f32, &asset_id);
        
        let size_as_f32: f32 = amount.value() * status_level.value();
        let size_as_unit: Unit = status_risk.unit_type().create_unit(size_as_f32);
        let size: PositionSize = PositionSize(size_as_unit);

        PositionStatus {
            status_level: status_level,
            status_amount: amount,
            status_stop: status_stop_level,
            status_risk: status_risk,
            status_size: size,
        }
    }
    pub(crate) fn update_with_self(self) -> Self {
        todo!()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PositionSize(Unit);
impl PositionSize {
    pub(crate) fn get_value(&self) -> &Unit {
        &self.0
    }
}
