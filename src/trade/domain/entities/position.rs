use std::marker::PhantomData;

use crate::{
    shared::{
        ids::{PositionId, SessionId},
        objects::{common::CommonDateTime, unit::Unit, *},
    },
    trade::domain::{
        entities::{asset::SessionAsset, trade::BaseTradeLog},
        objects::position::{PositionStatus, PositionType},
        states::TradeLog,
    },
};

pub(crate) struct BaseTradePosition<State: Persistency> {
    id: PositionId,
    session_id: SessionId,
    asset: SessionAsset,
    status: PositionStatus,
    direction: PositionType,
    state: OpenClosedState,
    logs: Vec<TradeLog>,
    _state: PhantomData<State>,
}
impl<State: Persistency> BaseTradePosition<State> {
    
    pub(crate) fn open(log: BaseTradeLog<Unpersisted>) -> BaseTradePosition<Unpersisted> {
        todo!()
    }
    pub(crate) fn apply_log(log: BaseTradeLog<Unpersisted>) -> BaseTradePosition<Unpersisted> {
        todo!()
    }
    pub(crate) fn close(log: BaseTradeLog<Unpersisted>) -> ClosedTradePosition<Unpersisted> {
        todo!()
    }
}
pub(crate) struct ClosedTradePosition<State: Persistency> {
    base: BaseTradePosition<State>,
    closed_time: CommonDateTime,
    total_pnl: Unit,
    _state: PhantomData<State>,
}
