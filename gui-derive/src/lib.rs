extern crate proc_macro;

mod lenses;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Generates lenses to access the fields of a struct
///
/// An associated constant is defined on the struct for each field,
/// having the same name as the field.
#[proc_macro_derive(Lenses)]
pub fn derive_lens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    lenses::derive_lens_impl(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
