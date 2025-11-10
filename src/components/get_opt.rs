use super::prelude::*;

#[derive(Debug, Default)]
pub struct Opt {
    pub name: Option<TokenStream>,
    pub pattern: Option<TokenStream>,
    pub err: Option<TokenStream>,
    pub default: Option<TokenStream>,
    pub min: Option<TokenStream>,
    pub max: Option<TokenStream>,
    pub err_min: Option<TokenStream>,
    pub err_max: Option<TokenStream>,
    pub aes_secret_key: Option<TokenStream>,
    pub chacha_secret_key: Option<TokenStream>,
    pub argon2_params: Option<TokenStream>,
}

pub fn get_opt(attributes: &Vec<Attribute>) -> Opt {
    let mut opt = Opt::default();

    for attr in attributes {
        if attr.path().is_ident("opt") {
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    match ident.to_string().as_str() {
                        "name" => {
                            if let Ok(ts) = meta.value()?.parse::<proc_macro2::TokenStream>() {
                                opt.name = Some(ts);
                            }
                        }
                        "pattern" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.pattern = Some(quote! { #expr });
                            }
                        }
                        "err" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.err = Some(quote! { #expr });
                            }
                        }
                        "default" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.default = Some(quote! { #expr });
                            }
                        }
                        "min" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.min = Some(quote! { #expr });
                            }
                        }
                        "max" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.max = Some(quote! { #expr });
                            }
                        }
                        "err_min" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.err_min = Some(quote! { #expr });
                            }
                        }
                        "err_max" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.err_max = Some(quote! { #expr });
                            }
                        }
                        "aes_secret_key" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.aes_secret_key = Some(quote! { #expr });
                            }
                        }
                        "chacha_secret_key" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.chacha_secret_key = Some(quote! { #expr });
                            }
                        }
                        "argon2_params" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.argon2_params = Some(quote! { #expr });
                            }
                        }
                        _ => {}
                    }
                }

                Ok(())
            })
            .unwrap();
        }
    }

    opt
}
