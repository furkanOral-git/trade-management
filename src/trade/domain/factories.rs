use std::marker::PhantomData;

use rumt::{emit_event, prelude::RuntimeEvent};

use crate::{
    shared::{
        event::SessionCreatedEvent,
        ids::{PositionId, SessionId},
        objects::{Amount, CommonDateTime, PriceLevel, Unit},
        states::{OpenClosedState, Persisted, Unpersisted},
    },
    trade::domain::{
        TradeSession,
        entity::{SessionAsset, TradeLog},
        objects::{PositionType, TradeAction, TradeType},
        states::TradePositionPersistable,
    },
};

pub(crate) struct TradeSessionFactory;

impl TradeSessionFactory {
    pub(crate) fn create_new(
        name: impl Into<String>,
        white_list: impl Into<Vec<SessionAsset>>,
        capital: Unit,
    ) -> TradeSession<Unpersisted> {
        let open_time = CommonDateTime::now();
        let session = TradeSession {
            id: SessionId(0),
            name: name.into(),
            open_time: open_time,
            state: OpenClosedState::Opened,
            capital: capital,
            closed_time: None,
            positions: vec![],
            white_list: white_list.into(),
            _state: PhantomData,
        };
        session
    }
    pub(crate) fn from_db(
        id: SessionId,
        name: impl Into<String>,
        open_time: CommonDateTime,
        state: OpenClosedState,
        capital: Unit,
        closed_time: Option<CommonDateTime>,
        positions: impl Into<Vec<TradePositionPersistable>>,
        white_list: impl Into<Vec<SessionAsset>>,
    ) -> TradeSession<Persisted> {
        TradeSession {
            id: id,
            name: name.into(),
            open_time: open_time,
            state: state,
            capital: capital,
            closed_time: closed_time,
            positions: positions.into(),
            white_list: white_list.into(),
            _state: PhantomData,
        }
    }
}
pub(crate) struct TradeLogFactory;
impl TradeLogFactory {
    pub(crate) fn new_unbounded(
        session_id: SessionId,
        action: TradeAction,
        at_level: PriceLevel,
        amount: Amount,
    ) -> TradeLog<Unpersisted> {
        todo!()
    }
    pub(crate) fn new_bounded(
        position_id: PositionId,
        session_id: SessionId,
        action: TradeAction,
        at_level: PriceLevel,
        amount: Amount,
    ) -> TradeLog<Unpersisted> {
        todo!()
    }
}

pub(crate) struct TradePositionFactory;
impl TradePositionFactory {}
