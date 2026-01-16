use crate::shared::{ids::PositionId, objects::{common::CommonDateTime, unit::Unit}};


#[derive(Debug)]
pub(crate) struct SessionCreatedEvent {
    pub(crate) name: String,
    pub(crate) capital: Unit,
    pub(crate) date_time : CommonDateTime,
}
pub(crate) struct PositionClosedEvent {
    pub(crate) id: PositionId,
    pub(crate) pnl: Unit,
}