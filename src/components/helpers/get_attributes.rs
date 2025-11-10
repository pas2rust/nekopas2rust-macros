pub use super::import::*;

pub fn get_attributes(input: &DeriveInput) -> Vec<Attribute> {
    input.attrs.clone()
}
