use std::marker::PhantomData;

use crate::{shared::{ids::{PositionId, SessionId}, objects::{Persistency, asset::*, common::{CommonDateTime, Description}}}, trade::domain::objects::trade::{TradeAction, TradeType}};

pub(crate) struct TradeLog<State: Persistency> {
    position_id: PositionId,
    session_id: SessionId,
    trade_type: TradeType,
    action: TradeAction,
    level: AssetPriceLevel,
    amount: AssetAmount,
    date_time: CommonDateTime,
    description: Description,
    _state: PhantomData<State>,
}