use inflector::cases::{classcase::*, snakecase::*};
use proc_macro2::{Ident, Span};
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

    let twizzled_name = if is_class_case(&ty.to_string()) {
        let temp_name = format!("{}_derived_lenses", to_snake_case(&ty.to_string()));
        Ident::new(&temp_name, Span::call_site())
    } else {
        return Err(syn::Error::new(
            ty.span(),
            "Lens implementations can only be derived from CamelCase types",
        ));
    };

    let lenses = fields
        .into_iter()
        .map(|f| {
            let field_name = f.clone().ident.unwrap();
            let lens_name = format!(
                "{}{}Lens",
                ty.to_string(),
                to_class_case(&field_name.to_string())
            );
            let field_ty = f.ty.clone();
            (
                field_name,
                Ident::new(&lens_name, Span::call_site()),
                field_ty,
            )
        })
        .collect::<Vec<_>>();

    // Define lens types for each field
    let defs = lenses.iter().map(|(_, lens_name, _)| {
        quote! {
            /// Lens for the field on #ty
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub struct #lens_name;
        }
    });

    let impls = lenses.iter().map(|(field_name, lens_name, field_ty)| {
        let err_str = format!(
            "Downcast error in {} - could not downcast to {}",
            lens_name, ty
        );
        let target_str = format!("{}::{}", ty, field_name);
        quote! {
            impl crate::Lens for #twizzled_name::#lens_name {
                type Source = Widget;
                type Target = #field_ty;

                fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target {
                    &source.inner.downcast_ref::<#ty>().expect(#err_str)
                        .#field_name
                }
                fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target {
                    &mut source.inner.downcast_mut::<#ty>().expect(#err_str)
                        .#field_name
                }
            }
            impl crate::LeafLens for #twizzled_name::#lens_name {
                fn target(&self) -> String {
                    #target_str.to_string()
                }
            }
        }
    });

    let associated_items = lenses.iter().map(|(field_name, lens_name, _)| {
        let doc = format!(
            "[LeafLens] to access the field `{}` of `{}`",
            field_name, ty
        );
        quote! {
            #[doc = #doc]
            pub const #field_name: #twizzled_name::#lens_name = #twizzled_name::#lens_name;
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
