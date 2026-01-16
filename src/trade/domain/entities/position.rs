use std::marker::PhantomData;

use crate::{
    shared::{
        ids::{PositionId, SessionId},
        objects::{common::CommonDateTime, unit::Unit, *},
    },
    trade::domain::{
        entities::{asset::SessionAsset, trade::TradeLog},
        objects::position::{PositionStatus, PositionType},
        states::TradeLogPersistable,
    },
};

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
