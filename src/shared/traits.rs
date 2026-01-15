use std::fmt::Error;

pub trait AggregateRoot {}
pub trait DomainEntity {}
pub trait ValueObject {}

pub trait BaseIdTrait: ValueObject {}
pub trait DomainError {
    fn message(&self) -> &str;
}
pub trait Repository<T: AggregateRoot, I: BaseIdTrait> {
    fn insert(&mut self, entity: T) -> Result<(), Error>;
    fn remove_by_id(&mut self, entity_id: I) -> Result<(), Error>;
    fn persist(&mut self, entity: T) -> Result<(), Error>;
    fn get_by_id(&mut self, id: I) -> Result<T, Error>;
    fn get_all(&mut self) -> Result<Vec<T>, Error>;
}
