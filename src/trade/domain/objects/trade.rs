#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TradeType {
    Buy,
    Sell,
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TradeAction {
    PartialClose,
    FullClose,
    AddSize,
    Initial,
}