use crate::{impl_display_trait, shared::traits::DomainError};
#[derive(Debug)]
pub(crate) enum TradeSessionDomainError {
    ClosedTradeSessionCanNotOpenPosition,
}
impl DomainError for TradeSessionDomainError {
    fn message(&self) -> &str {
        match self {
            TradeSessionDomainError::ClosedTradeSessionCanNotOpenPosition => {
                "TradeSessionError: Closed Session's can not open new position!"
            }
        }
    }
}
impl_display_trait!(TradeSessionDomainError);

pub(crate) enum TradePositionDomainError {
    ClosedPositionCanNotClosedAgain,
}
impl DomainError for TradePositionDomainError {
    fn message(&self) -> &str {
        match self {
            TradePositionDomainError::ClosedPositionCanNotClosedAgain => {
                "TradeSessionError : Position has already been closed!"
            }
        }
    }
}
impl_display_trait!(TradePositionDomainError);
