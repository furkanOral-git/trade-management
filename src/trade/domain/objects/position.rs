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
}

impl PositionStatus {
    pub(crate) fn create_status(
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

        PositionStatus {
            status_cost_level: cost_level,
            status_amount: amount,
            status_stop: stop_level,
            status_risk: risk,
            status_size: size,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PositionType {
    Long,
    Short,
}
