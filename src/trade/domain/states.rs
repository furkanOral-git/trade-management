use crate::{
    shared::states::{Persisted, Unpersisted},
    trade::domain::{TradeSession, entity::{TradeLog, TradePosition}},
};
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