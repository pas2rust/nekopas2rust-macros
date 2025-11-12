mod components;
#[cfg(any(
    feature = "builder",
    feature = "parser",
    feature = "cipher",
    feature = "math",
    feature = "print"
))]
use components::prelude::*;

#[cfg(feature = "builder")]
#[proc_macro_derive(Builder, attributes(opt))]
pub fn builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generate = build_app(&input);
    generate.into()
}

#[cfg(feature = "print")]
#[proc_macro_derive(Print, attributes(transporter))]
pub fn print(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut expanded = proc_macro2::TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![print_by_field(&input), print_method(&input)],
    );
    expanded.into()
}

#[cfg(feature = "math")]
#[proc_macro_derive(Math)]
pub fn math(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_math = math_app(&input);
    generate_math.into()
}

#[cfg(feature = "cipher")]
#[proc_macro_derive(Cipher, attributes(opt))]
pub fn cipher(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_cipher = cipher_app(&input);
    generate_cipher.into()
}

#[cfg(feature = "parser")]
#[proc_macro_derive(Parser, attributes(opt))]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_into = parser_app(&input);
    generate_into.into()
}
