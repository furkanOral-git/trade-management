use crate::shared::{
    ids::{PositionId, SessionId},
    objects::Unit,
};
#[derive(Debug)]
pub(crate) struct SessionCreatedEvent {
    pub(crate) id: SessionId,
    pub(crate) name: String,
    pub(crate) capital: Unit,
}
pub(crate) struct PositionClosedEvent {
    pub(crate) id: PositionId,
    pub(crate) pnl: Unit,
}
