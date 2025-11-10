use super::import::*;

pub fn new_ident(prefix: &str, field_name: &Ident) -> Ident {
    Ident::new(&format!("{}_{}", prefix, field_name), Span::call_site())
}
