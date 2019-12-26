use quote::quote;
use syn::{spanned::Spanned, Data};

pub(crate) fn derive_lens_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(_) => derive_struct(&input),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Lens implementations cannot be derived from enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Lens implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ty = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        return Err(syn::Error::new(
            input.span(),
            "Lens implementations can only be derived from structs with named fields",
        ));
    };

    let twizzled_name = if is_camel_case(&ty.to_string()) {
        let temp_name = format!("{}_derived_lenses", to_snake_case(&ty.to_string()));
        proc_macro2::Ident::new(&temp_name, proc_macro2::Span::call_site())
    } else {
        return Err(syn::Error::new(
            ty.span(),
            "Lens implementations can only be derived from CamelCase types",
        ));
    };

    // Define lens types for each field
    let defs = fields.iter().map(|f| {
        let field_name = &f.ident;

        quote! {
            /// Lens for the field on #ty
            #[allow(non_camel_case_types)]
            pub struct #field_name;
            // TODO  camel case name?
        }
    });

    let impls = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_ty = &f.ty;

        quote! {
            impl crate::FieldLens for #twizzled_name::#field_name {
                type Target = #field_ty;

                fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target {
                    &source.inner.downcast_ref::<#ty>().unwrap()
                        .#field_name
                }
                fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target {
                    &mut source.inner.downcast_mut::<#ty>().unwrap()
                        .#field_name
                }

            }
        }
    });

    let associated_items = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            /// Lens for the corresponding field
            pub const #field_name: #twizzled_name::#field_name = #twizzled_name::#field_name;
        }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        pub mod #twizzled_name {
            #(#defs)*
        }

        #(#impls)*

        #[allow(non_upper_case_globals)]
        impl #impl_generics #ty #ty_generics #where_clause {
            #(#associated_items)*
        }
    };

    Ok(expanded)
}

//I stole these from rustc!
fn char_has_case(c: char) -> bool {
    c.is_lowercase() || c.is_uppercase()
}

fn is_camel_case(name: &str) -> bool {
    let name = name.trim_matches('_');
    if name.is_empty() {
        return true;
    }

    // start with a non-lowercase letter rather than non-uppercase
    // ones (some scripts don't have a concept of upper/lowercase)
    !name.chars().next().unwrap().is_lowercase()
        && !name.contains("__")
        && !name.chars().collect::<Vec<_>>().windows(2).any(|pair| {
            // contains a capitalisable character followed by, or preceded by, an underscore
            char_has_case(pair[0]) && pair[1] == '_' || char_has_case(pair[1]) && pair[0] == '_'
        })
}

fn to_snake_case(mut str: &str) -> String {
    let mut words = vec![];
    // Preserve leading underscores
    str = str.trim_start_matches(|c: char| {
        if c == '_' {
            words.push(String::new());
            true
        } else {
            false
        }
    });
    for s in str.split('_') {
        let mut last_upper = false;
        let mut buf = String::new();
        if s.is_empty() {
            continue;
        }
        for ch in s.chars() {
            if !buf.is_empty() && buf != "'" && ch.is_uppercase() && !last_upper {
                words.push(buf);
                buf = String::new();
            }
            last_upper = ch.is_uppercase();
            buf.extend(ch.to_lowercase());
        }
        words.push(buf);
    }
    words.join("_")
}
