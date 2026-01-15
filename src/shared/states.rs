#[derive(Debug,PartialEq,Clone)]
pub enum OpenClosedState {
    Opened,
    Closed,
}

//Oluşturulma,Güncellenme durumlarında Unpersisted
//DB'den çekildikten sonra, repository'de persist edildikten sonra Persisted
pub(crate) struct Unpersisted;
pub(crate) struct Persisted;

pub(crate) trait Persistency {}
impl Persistency for Unpersisted {}
impl Persistency for Persisted {}


