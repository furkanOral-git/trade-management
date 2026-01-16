pub(crate) struct AssetName(pub(crate) String);

impl AssetName {
    pub fn new(name: impl Into<String>) -> Self {
        //uzunluk kontrolü, UPPER_CASE hale getirme, diğer kontroller.
        AssetName(name.into())
    }
}