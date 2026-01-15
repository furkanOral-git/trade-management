#![allow(unused)]

use std::{fmt::Error, marker::PhantomData};

use rumt::{emit_event, prelude::RuntimeEvent};

use crate::{
    shared::{
        event::SessionCreatedEvent,
        ids::*,
        objects::{Amount, CommonDateTime, PriceLevel, Unit},
        states::{OpenClosedState, Persisted, Persistency, Unpersisted}, traits::DomainError,
    },
    trade::domain::{
        entity::{ClosedTradePosition, SessionAsset, TradeLog, TradePosition}, errors::TradeSessionDomainErrors, objects::{PositionType, TradeType}, states::TradePositionPersistable
    },
};

pub mod entity;
pub mod objects;
pub mod states;
pub mod errors;
pub mod factories;

pub(crate) struct TradeSession<State: Persistency> {
    id: SessionId,
    name: String,
    open_time: CommonDateTime,
    state: OpenClosedState,
    capital: Unit,
    closed_time: Option<CommonDateTime>,
    positions: Vec<TradePositionPersistable>,
    white_list: Vec<SessionAsset>,
    _state: PhantomData<State>,
}

impl<State: Persistency> TradeSession<State> {
    // Aggregate'in methodları içerisine yalnızca VO'lar gelebilir. Entity'ler parametre olarak gelemez.
    // Çünkü Application katmanının sınırları ihlal edilmiş olur. Application sadece uygulama katındaki loglama, cachleme, repo, exception handling gibi
    // ya da dış servislerin implementasyonu için kullanılır. Application yalnızca AR ya da Birden fazla AR'ı bir arada çalıştıran Domain Service kullanabilir.
    // Entity'i application katmanında oluşturmak bu sınırı ihlal eder ve DDD yapısı bozulur.
    
    pub(crate) fn open_new_position(
        &mut self,
        direction : PositionType,
        at_level : PriceLevel,
        stop_level: PriceLevel,
        risk : Unit

    ) -> Result<TradeSession<Unpersisted>,TradeSessionDomainErrors> {
        if self.state == OpenClosedState::Closed{
            let err = TradeSessionDomainErrors::ClosedTradeSessionCanNotOpenPosition;
            return Err(err)
        }
        todo!()

    }
    pub(crate) fn close_position(
        &mut self,
        posiiton_id: PositionId,
        at_level : PriceLevel,
    ) -> TradeSession<Unpersisted> {
        todo!()
    }
    pub(crate) fn close_position_partial(
        &mut self,
        log: TradeLog<Unpersisted>,
    ) -> TradeSession<Unpersisted> {
        todo!()
    }
    pub(crate) fn close_session(&mut self) -> TradeSession<Unpersisted> {
        todo!()
    }
}
