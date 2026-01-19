use crate::{
    shared::objects::{Persisted, Unpersisted},
    trade::domain::{BaseTradeSession, entities::{
        position::{BaseTradePosition},
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
}

pub(crate) enum TradeLog {
    Persisted(BaseTradeLog<Persisted>),
    Unpersisted(BaseTradeLog<Unpersisted>),
}
impl TradeLog{
    pub(crate) fn is_persisted(&self)->bool{
        match self {
            TradeLog::Persisted(base_trade_log) => true,
            TradeLog::Unpersisted(base_trade_log) => false,
        }
    }
}
