use crate::{shared::objects::{Persisted, Unpersisted}, trade::domain::{TradeSession, entities::{position::TradePosition, trade::TradeLog}}};
pub(crate) enum TradeSessionPersistable{
    Persisted(TradeSession<Persisted>),
    Unpersisted(TradeSession<Unpersisted>)
}
pub(crate) enum TradePositionPersistable {
    Persisted(TradePosition<Persisted>),
    Unpersisted(TradePosition<Unpersisted>),
}

pub(crate) enum TradeLogPersistable{
    Persisted(TradeLog<Persisted>),
    Unpersisted(TradeLog<Unpersisted>),
}