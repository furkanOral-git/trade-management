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
        let amount = PositionStatus::calc_amount(&cost_level, &stop_level, &risk);
        let size = PositionStatus::calc_size(&cost_level, &amount);
        let pnl = Unit::new(0.0);

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
    ) -> Self {
        let new_amount = Self::calc_amount(&cost_level, &stop_level, &risk);
        let mut calculated_amount: f32 = 0.0;
        let calculated_cost_level = Self::calc_cost_level(
            &self.status_cost_level,
            &self.status_amount,
            &cost_level,
            &new_amount,
            &mut calculated_amount,
        );
        let amount = AssetAmount::new(calculated_amount);
        let calculated_risk = Self::calc_risk(&amount, &calculated_cost_level, &stop_level);
        let size = Self::calc_size(&calculated_cost_level, &amount);
        
        PositionStatus{
            status_amount : amount,
            status_cost_level : calculated_cost_level,
            status_risk : calculated_risk,
            status_size : size,
            status_stop : stop_level,
            status_pnl : self.status_pnl,
        }
    }
    pub(crate) fn update_for_close(self, at_level: AssetPriceLevel, amount: AssetAmount) -> Self {
        todo!()
    }
}
impl PositionStatus {
    fn calc_amount(
        cost_level: &AssetPriceLevel,
        stop_level: &AssetPriceLevel,
        risk: &Unit,
    ) -> AssetAmount {
        let range = cost_level.calc_range(stop_level);
        //amount (x) * range = risk
        // risk / range = amount
        let amount_as_f32 = risk.value() / range.value();
        AssetAmount::new(amount_as_f32)
    }
    fn calc_size(cost_level: &AssetPriceLevel, amount: &AssetAmount) -> Unit {
        // size = cost_level * amount
        let size_as_f32 = cost_level.value() * amount.value();
        Unit::new(size_as_f32)
    }
    fn calc_cost_level(
        old_cost: &AssetPriceLevel,
        old_amount: &AssetAmount,
        new_cost: &AssetPriceLevel,
        new_amount: &AssetAmount,
        calc_amount_ref: &mut f32,
    ) -> AssetPriceLevel {
        /*
            Yeni maaliyet seviyesi =
            [(Eski Maaliyet Seviyesi * Eski Lot) + (Yeni Maaliyet Seviyesi * Yeni Lot)] / (Eski Lot + Yeni Lot)
        */
        let upper =
            (old_cost.value() * old_amount.value()) + (new_cost.value() * new_amount.value());
        let mut under = old_amount.value() + new_amount.value();
        let new_cost_level = &upper / &under;
        *calc_amount_ref = under;

        AssetPriceLevel::new(new_cost_level)
    }
    fn calc_risk(
        total_amount: &AssetAmount,
        cost_level: &AssetPriceLevel,
        stop_level: &AssetPriceLevel,
    ) -> Unit {
        //amount * range = risk

        let range = cost_level.calc_range(stop_level);
        let risk_as_f32 = range.value() * total_amount.value();
        Unit::new(risk_as_f32)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PositionType {
    Long,
    Short,
}
