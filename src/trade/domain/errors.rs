use crate::{impl_display_trait, shared::traits::DomainError};
#[derive(Debug)]
pub(crate) enum TradeSessionDomainErrors{
    ClosedTradeSessionCanNotOpenPosition,
}
impl DomainError for TradeSessionDomainErrors{
    fn message(&self) -> &str {
        match self{
            TradeSessionDomainErrors::ClosedTradeSessionCanNotOpenPosition => "TradeSessionError: Closed Session's can not open new position!",
        }
    }
}
impl_display_trait!(TradeSessionDomainErrors);