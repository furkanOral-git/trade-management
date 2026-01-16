use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct CommonDateTime(pub(crate) DateTime<Utc>);
impl CommonDateTime {
    pub(crate) fn now() -> CommonDateTime {
        CommonDateTime(Utc::now())
    }
}



pub(crate) struct Description(pub(crate) String);