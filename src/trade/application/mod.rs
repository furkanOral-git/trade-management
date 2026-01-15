use crate::trade::application::repositories::TradeSessionRepository;

pub mod records;
pub mod repositories;
pub mod queries;
pub mod commands;

pub struct SessionService{
    repo  : TradeSessionRepository,
}