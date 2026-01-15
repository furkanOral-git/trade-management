#[macro_export]
macro_rules! define_id {
    ($struct_name:ident, $value_type:ty) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $struct_name(pub $value_type);
        impl $crate::shared::traits::ValueObject for $struct_name {}
        impl $crate::shared::traits::BaseIdTrait for $struct_name {}

        // Optional: Implement From to make creation easier
        impl From<$value_type> for $struct_name {
            fn from(value: $value_type) -> Self {
                Self(value)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_display_trait {
    ($struct_name:ty) => {
        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.message())
            }
        }
    };
}
