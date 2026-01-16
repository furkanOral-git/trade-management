use crate::shared::objects::{amount::Amount, unit::{Unit, UnitType}};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct PriceLevel {
    value: f32,
    value_type_as_unit : UnitType
}
impl PriceLevel {
    pub(crate) fn value(&self) -> &f32 {
        &self.value
    }
    pub(crate) fn calc_range(&self, level: &PriceLevel) -> f32 {
        let mut range = self.value - level.value();
        range = if range < 0.0 { -range } else { range };
        range
    }
    pub(crate) fn calc_size_as_unit(&self,amount : &Amount)->Unit{
        let size_as_f32 = self.value * amount.value();
        self.value_type_as_unit.create_unit(size_as_f32)
    }
}