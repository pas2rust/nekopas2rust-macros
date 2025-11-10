mod components;
use components::prelude::*;

#[proc_macro_derive(Builder, attributes(opt))]
pub fn builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generate = build_app(&input);
    generate.into()
}

#[proc_macro_derive(Print, attributes(transporter))]
pub fn print(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut expanded = proc_macro2::TokenStream::new().into();
    for_extend_token_stream(
        &mut expanded,
        vec![print_by_field(&input), print_method(&input)],
    );
    expanded.into()
}

#[proc_macro_derive(Math)]
pub fn math(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_math = math_app(&input);
    generate_math.into()
}

#[proc_macro_derive(Cipher, attributes(opt))]
pub fn cipher(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_cipher = cipher_app(&input);
    generate_cipher.into()
}

#[proc_macro_derive(Parser, attributes(opt))]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_into = parser_app(&input);
    generate_into.into()
}

/*
#[proc_macro_derive(Sphere, attributes(opt, transporter))]
pub fn sphere(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let b = builder(input.clone()).into();
    let p = print(input.clone()).into();
    let m = math(input.clone()).into();
    let c = cipher(input).into();
    let mut expanded = proc_macro2::TokenStream::new().into();
    for_extend_token_stream(&mut expanded, vec![b, p, m, c]);
    expanded.into()
}
*/
