#![allow(unused)]

use std::{fmt::Error, marker::PhantomData};

use rumt::{emit_event, prelude::RuntimeEvent};

use crate::{shared::{events::SessionCreatedEvent, ids::{PositionId, SessionId}, objects::{OpenClosedState, Persistency, Unpersisted, amount::Amount, common::CommonDateTime, price::PriceLevel, unit::Unit}}, trade::domain::{entities::{asset::SessionAsset, position::TradePosition}, errors::{TradePositionDomainError, TradeSessionDomainError}, factories::{session::TradeSessionFactory, trade::TradeLogFactory}, objects::position::PositionType, states::TradePositionPersistable}};


pub mod entities;
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
    
    pub(crate) async fn open_new_session(name : &str,list : Vec<SessionAsset>,capital : Unit)->TradeSession<Unpersisted>{
        
        let session = TradeSessionFactory::create_new(name, list, capital);
        
        let event_arg = SessionCreatedEvent {
            name: session.name.clone(),
            capital: session.capital.clone(),
            date_time : session.open_time.clone(),
        };
        let event = RuntimeEvent::Static {
            event_name: "Session.Created".into(),
        };
        emit_event(event, event_arg).await;
        session

    }
    // Aggregate'in methodları içerisine yalnızca VO'lar gelebilir. Entity'ler parametre olarak gelemez.
    // Çünkü Application katmanının sınırları ihlal edilmiş olur. Application sadece uygulama katındaki loglama, cachleme, repo, exception handling gibi
    // ya da dış servislerin implementasyonu için kullanılır. Application yalnızca AR ya da Birden fazla AR'ı bir arada çalıştıran Domain Service kullanabilir.
    // Entity'i application katmanında oluşturmak bu sınırı ihlal eder ve DDD yapısı bozulur.
    
    pub(crate) fn open_new_position(
        &mut self,
        direction : PositionType,
        at_level : PriceLevel,
        stop_level: PriceLevel,
        risk : Unit,

    ) -> Result<TradePosition<Unpersisted>,TradeSessionDomainError> {
        
        if self.state == OpenClosedState::Closed{
            let err = TradeSessionDomainError::ClosedTradeSessionCanNotOpenPosition;
            return Err(err)
        }
        //let log = TradeLogFactory::new_unbounded(self.id);
        todo!()

    }
    pub(crate) fn close_position(
        &mut self,
        position_id: PositionId,
        at_level : PriceLevel,
    ) -> Result<TradeSession<Unpersisted>,TradePositionDomainError> {
        todo!()
    }
    pub(crate) fn close_position_partial(
        &mut self,
        position_id : PositionId,
        at_level : PriceLevel,
        amount : Amount,
    ) -> TradeSession<Unpersisted> {
        todo!()
    }
    pub(crate) fn close_session(&mut self) -> TradeSession<Unpersisted> {
        todo!()
    }
}
