use std::thread::current;

use crate::shared::{
    ids::AssetId,
    objects::{asset::*, unit::Unit},
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PositionStatus {
    status_cost_level: AssetPriceLevel,
    status_amount: AssetAmount,
    status_stop: AssetPriceLevel,
    status_risk: Unit,
    status_size: Unit,
    status_pnl: Unit,
}

impl PositionStatus {
    pub(crate) fn new(
        cost_level: AssetPriceLevel,
        stop_level: AssetPriceLevel,
        risk: Unit,
    ) -> Self {
        let range = cost_level.calc_range(&stop_level);
        //amount (x) * range = risk
        // risk / range = amount
        let amount: AssetAmount =
            AssetAmount::new(risk.value() / range.value(), cost_level.asset_id().clone());

        // size = cost_level * amount
        let size_as_f32 = cost_level.value() * amount.value();
        let size = Unit::new(cost_level.currency().clone(), size_as_f32);
        let pnl = Unit::new(cost_level.currency().clone(), 0.0);

        PositionStatus {
            status_cost_level: cost_level,
            status_amount: amount,
            status_stop: stop_level,
            status_risk: risk,
            status_size: size,
            status_pnl: pnl,
        }
    }
    pub(crate) fn update_for_add_size(
        self,
        cost_level: AssetPriceLevel,
        stop_level: AssetPriceLevel,
        risk: Unit,
        amount: AssetAmount,
    ) -> Self {
        todo!()
    }
    pub(crate) fn update_for_close(self, at_level: AssetPriceLevel, amount: AssetAmount) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PositionType {
    Long,
    Short,
}
