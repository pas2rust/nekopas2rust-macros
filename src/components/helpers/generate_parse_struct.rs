pub use super::import::*;

pub fn generate_parse_struct() -> TokenStream {
    quote! {
        pub struct Parse<T>(pub T);
    }
}
