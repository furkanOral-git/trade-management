use chrono::{DateTime, Utc};
#[derive(Debug, Clone, PartialEq)]
pub struct CommonDateTime(pub(crate) DateTime<Utc>);
impl CommonDateTime {
    pub(crate) fn now() -> CommonDateTime {
        CommonDateTime(Utc::now())
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Description(String);
impl Description {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Description(value.into())
    }
    pub(crate) fn value(&self) -> &String {
        &self.0
    }
}
