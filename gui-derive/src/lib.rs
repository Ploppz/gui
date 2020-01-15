extern crate proc_macro;

mod lenses;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::parse_macro_input;

/// Generates lenses to access the fields of a struct
///
/// An associated constant is defined on the struct for each field,
/// having the same name as the field.
///
/// # Current shortcomings / TODO's
/// ## Generics
/// Basic generics work but the proc macro will ignore any where clause in the struct.
///
#[proc_macro_derive(Lens, attributes(lens))]
pub fn derive_lens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    lenses::derive_lens_impl(input, Ident::new("gui", Span::call_site()))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(LensInternal, attributes(lens))]
pub fn derive_lens_internal(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let x = lenses::derive_lens_impl(input, Ident::new("crate", Span::call_site()))
        .unwrap_or_else(|err| err.to_compile_error())
        .into();
    println!("{}", x);
    x
}
