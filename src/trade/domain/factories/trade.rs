use crate::{
    shared::{
        ids::{PositionId, SessionId},
        objects::{Unpersisted, amount::Amount, price::PriceLevel},
    },
    trade::domain::{entities::trade::TradeLog, objects::trade::TradeAction},
};

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
