use super::prelude::*;

pub fn parser_app(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_clone = input.clone();
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;
    let impl_block = get_impl(input);
    let struct_name = get_struct_name(input);
    let key_enum_ident = format_ident!("{struct_name}ParserKey");
    let value_enum_ident = format_ident!("{struct_name}ParserValue");
    let field_enum_idents: Vec<Ident> = fields
        .iter()
        .map(|f| {
            let field_captalize: String = f
                .ident
                .as_ref()
                .expect("field name must be set")
                .to_string()
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect();
            format_ident!("{struct_name}{field_captalize}")
        })
        .collect();
    let field_types: Vec<Type> = fields.iter().map(|f| f.ty.clone()).collect();
    let field_idents: Vec<Ident> = fields
        .iter()
        .map(|f| f.ident.as_ref().expect("field name must be set").clone())
        .collect();
    let type_hash_map_ident = format_ident!("{struct_name}HashMap");
    let type_hash_set_ident = format_ident!("{struct_name}HashSet");
    quote! {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum #key_enum_ident {
            #(#field_enum_idents),*
        }
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum #value_enum_ident {
            #(#field_enum_idents(#field_types)),*
        }
        pub type #type_hash_map_ident = std::collections::HashMap<#key_enum_ident, #value_enum_ident>;
        pub type #type_hash_set_ident = std::collections::HashSet<#value_enum_ident>;

        impl #impl_block {
            pub fn to_hash_map(self) -> #type_hash_map_ident {
                self.into()
            }
            pub fn to_hash_set(self) -> #type_hash_set_ident {
                self.into()
            }
            pub fn to_bincode(self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
                self.try_into()
            }
            pub fn from_bincode(bytes: Vec<u8>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                Self::try_from(bytes)
            }
        }

        impl TryFrom<Vec<u8>> for #impl_block {
            type Error = Box<dyn std::error::Error + Send + Sync>;
            fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                let (v, _len) = bincode::decode_from_slice(&bytes[..], bincode::config::standard())
                    .map_err(|e| Box::new(e))?;
                Ok(v)
            }
        }

        impl TryInto<Vec<u8>> for #impl_block {
            type Error = Box<dyn std::error::Error + Send + Sync>;
            fn try_into(self) -> Result<Vec<u8>, Self::Error> {
                let bytes = bincode::encode_to_vec(&self, bincode::config::standard())
                    .map_err(|e| Box::new(e))?;
                Ok(bytes)
            }
        }

        impl Into<#type_hash_set_ident> for #impl_block {
            fn into(self) -> #type_hash_set_ident {
                let #struct_name { #(#field_idents),* } = self;
                let mut set = std::collections::HashSet::new();
                #(
                    set.insert(#value_enum_ident::#field_enum_idents(#field_idents));
                )*
                set
            }
        }

        impl Into<#type_hash_map_ident> for #impl_block {
            fn into(self) -> #type_hash_map_ident {
                let #struct_name { #(#field_idents),* } = self;
                let mut map = std::collections::HashMap::new();
                #(
                    map.insert(
                        #key_enum_ident::#field_enum_idents,
                        #value_enum_ident::#field_enum_idents(#field_idents)
                    );
                )*

                map
            }
        }
    }
}
