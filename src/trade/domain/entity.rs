use std::marker::PhantomData;

use crate::{
    shared::{
        ids::*,
        objects::{Amount, AssetName, CommonDateTime, Description, PriceLevel, Unit},
        states::{OpenClosedState, Persistency, Unpersisted},
    },
    trade::domain::{objects::*, states::TradeLogPersistable},
};

pub(crate) struct TradeLog<State: Persistency> {
    position_id: PositionId,
    session_id : SessionId,
    trade_type: TradeType,
    action: TradeAction,
    level: PriceLevel,
    amount: Amount,
    date_time: CommonDateTime,
    description: Description,
    _state: PhantomData<State>,
}
pub(crate) struct TradePosition<State: Persistency> {
    id: PositionId,
    session_id: SessionId,
    asset: SessionAsset,
    status: PositionStatus,
    direction: PositionType,
    state: OpenClosedState,
    logs: Vec<TradeLogPersistable>,
    _state: PhantomData<State>,
}
impl<State: Persistency> TradePosition<State> {
    pub(crate) fn close_position(
        &mut self,
        log: TradeLog<Unpersisted>,
    ) -> ClosedTradePosition<Unpersisted> {
        todo!()
    }
    pub(crate) fn close_partial(
        &mut self,
        log: TradeLog<Unpersisted>,
    ) -> TradePosition<Unpersisted> {
        todo!()
    }
    pub(crate) fn add_size(&mut self, log: TradeLog<Unpersisted>) -> TradePosition<Unpersisted> {
        todo!()
    }
}
pub(crate) struct ClosedTradePosition<State: Persistency> {
    base: TradePosition<State>,
    closed_time: CommonDateTime,
    total_pnl: Unit,
    _state: PhantomData<State>,
}
pub(crate) struct SessionAsset {
    id: AssetId,
    name: AssetName,
}
impl SessionAsset {
    pub(crate) fn new(id: AssetId, name: AssetName) -> Self {
        SessionAsset { id, name }
    }
    pub(crate) fn create_amount(&self, value: f32) -> Amount {
        Amount::new(value, &self.id)
    }
}
