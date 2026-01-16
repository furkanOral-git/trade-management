use crate::shared::{ids::AssetId, objects::{amount::Amount, asset::AssetName}};

pub(crate) struct SessionAsset {
    id: AssetId,
    name: AssetName,
}
impl SessionAsset {
    pub(crate) fn new(id: AssetId, name: AssetName) -> Self {
        SessionAsset { id, name }
    }
    pub(crate) fn create_amount(&self, value: f32) -> Amount {
        Amount::new(value, &self.id)
    }
}