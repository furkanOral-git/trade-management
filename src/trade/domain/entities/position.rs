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
    opening_date : CommonDateTime,
    closing_date : Option<CommonDateTime>,
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
    pub(crate) fn apply_log(
        &mut self,
        log: BaseTradeLog<Unpersisted>,
    ) -> BaseTradePosition<Unpersisted> {
        todo!()
    }
    pub(crate) fn is_open(&self)->bool{
        self.closing_date == None
    }
}
impl BaseTradePosition<Unpersisted> {
    ///usize : The end of the vector is assumed to be a starting index of 0, and 0 represents the last element of logs.
    pub(crate) fn peak_with_tail_index(&self, tail_index: usize) -> Option<&TradeLog> {
        let last = self.logs.len() + (tail_index - 1);
        self.logs.get(last)
    }
}

