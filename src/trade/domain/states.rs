use crate::{
    shared::objects::{Persisted, Unpersisted},
    trade::domain::{BaseTradeSession, entities::{
        position::{BaseTradePosition, ClosedTradePosition},
        trade::{BaseTradeLog},
    }},
};
pub(crate) enum TradeSession {
    Persisted(BaseTradeSession<Persisted>),
    Unpersisted(BaseTradeSession<Unpersisted>),
}
pub(crate) enum TradePosition {
    Persisted(BaseTradePosition<Persisted>),
    Unpersisted(BaseTradePosition<Unpersisted>),
    PersistedClosed(ClosedTradePosition<Persisted>),
    UnpersistedClosed(ClosedTradePosition<Unpersisted>),
}

pub(crate) enum TradeLog {
    Persisted(BaseTradeLog<Persisted>),
    Unpersisted(BaseTradeLog<Unpersisted>),
}
