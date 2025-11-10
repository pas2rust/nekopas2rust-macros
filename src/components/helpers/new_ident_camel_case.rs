use super::import::*;

pub fn new_ident_camel_case(prefix: &str, field_name: Ident) -> Ident {
    Ident::new(
        format!("{}{}", prefix, field_name).as_str(),
        field_name.span(),
    )
}
