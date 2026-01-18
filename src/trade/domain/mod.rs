#![allow(unused)]

use std::{fmt::Error, marker::PhantomData};

use rumt::{emit_event, prelude::RuntimeEvent};

use crate::{
    shared::{
        events::SessionCreatedEvent,
        ids::{AssetId, PositionId, SessionId},
        objects::{
            OpenClosedState, Persistency, Unpersisted, asset::*, common::CommonDateTime, unit::Unit,
        },
    },
    trade::domain::{
        entities::{asset::SessionAsset, position::BaseTradePosition},
        errors::{TradePositionDomainError, TradeSessionDomainError},
        factories::{session::TradeSessionFactory, trade::TradeLogFactory},
        objects::{position::PositionType, trade::*}, states::TradePosition,
        
    },
};

pub mod entities;
pub mod errors;
pub mod factories;
pub mod objects;
pub mod states;

pub(crate) struct BaseTradeSession<State: Persistency> {
    id: SessionId,
    name: String,
    open_time: CommonDateTime,
    state: OpenClosedState,
    capital: Unit,
    closed_time: Option<CommonDateTime>,
    positions: Vec<TradePosition>,
    white_list: Vec<SessionAsset>,
    _state: PhantomData<State>,
}

impl<State: Persistency> BaseTradeSession<State> {
    pub(crate) async fn open_new_session(
        name: &str,
        list: Vec<SessionAsset>,
        capital: Unit,
    ) -> BaseTradeSession<Unpersisted> {
        let session = TradeSessionFactory::create_base(name, list, capital);

        let event_arg = SessionCreatedEvent {
            name: session.name.clone(),
            capital: session.capital.clone(),
            date_time: session.open_time.clone(),
        };
        let event = RuntimeEvent::Static {
            event_name: "Session.Created".into(),
        };
        emit_event(event, event_arg).await;
        session
    }
    
    pub(crate) fn close_position(
        &mut self,
        position_id: PositionId,
        at_level: AssetPriceLevel,
    ) -> Result<BaseTradeSession<Unpersisted>, TradePositionDomainError> {
        todo!()
    }
    pub(crate) fn close_position_partial(
        &mut self,
        position_id: PositionId,
        at_level: AssetPriceLevel,
        amount: AssetAmount,
    ) -> Result<BaseTradeSession<Unpersisted>, TradePositionDomainError> {
        todo!()
    }
    pub(crate) fn close_session(&mut self) -> BaseTradeSession<Unpersisted> {
        todo!()
    }
}

