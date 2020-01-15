use inflector::cases::{classcase::*, snakecase::*};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{spanned::Spanned, Data};

pub(crate) fn derive_lens_impl(
    input: syn::DeriveInput,
    cr: Ident,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(_) => derive_struct(&input, cr),
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

fn derive_struct(
    input: &syn::DeriveInput,
    cr: Ident,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ty = &input.ident;
    // TODO: use `where_clause`. Merge with our where clauses (look at uses of
    // `lens_where_clauses`)
    let syn::Generics {
        params: ref generic_params,
        ref where_clause,
        ..
    } = input.generics;

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
        .filter(|f| {
            f.attrs
                .iter()
                .find(|attr| {
                    attr.path.segments.len() == 1
                        && attr
                            .path
                            .segments
                            .last()
                            .map(|x| x.ident == "lens")
                            .unwrap_or(false)
                })
                .is_some()
        })
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

    // Add markers to the lens data
    let markers = generic_params.iter().enumerate().filter_map(|(i, param)| {
        if let syn::GenericParam::Type(ty) = param {
            let id = Ident::new(&format!("_marker{}", i), Span::call_site());
            let ty = &ty.ident;
            Some((id, ty))
        } else {
            None
        }
    });
    let markers_def = markers
        .clone()
        .map(|(id, ty)| {
            quote! {pub #id: std::marker::PhantomData<#ty>}
        })
        .collect::<Vec<_>>();
    let markers_init = markers
        .map(|(id, _ty)| {
            quote! {#id: std::marker::PhantomData}
        })
        .collect::<Vec<_>>();

    // require that all parameters are 'static + Clone
    let lens_where_clauses = generic_params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(ty) = param {
                let ty = &ty.ident;
                Some(quote! {#ty: 'static + Clone + std::fmt::Debug + Sync + Send})
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let lense_quotes = lenses.iter().map(|(field_name, lens_name, field_ty)| {
        let err_str = format!(
            "Downcast error in {} - could not downcast to {}",
            lens_name, ty
        );
        let target_str = format!("{}::{}", ty, field_name);
        quote! {
            /// Lens for the field on #ty
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub struct #lens_name <#generic_params> {
                #(#markers_def),*
            }
            impl <#generic_params> #cr::lens::Lens for #twizzled_name::#lens_name <#generic_params>
                where #(#lens_where_clauses)*,
                    #ty<#generic_params>: Interactive
            {
                type Source = #cr::Widget;
                type Target = #field_ty;

                fn get<'a>(&self, source: &'a #cr::Widget) -> &'a Self::Target {
                    &source.inner.downcast_ref::<super::#ty <#generic_params>>().expect(#err_str)
                        .#field_name
                }
                fn get_mut<'a>(&self, source: &'a mut #cr::Widget) -> &'a mut Self::Target {
                    &mut source.inner.downcast_mut::<super::#ty <#generic_params>>().expect(#err_str)
                        .#field_name
                }
            }
            impl <#generic_params> #cr::lens::LeafLens for #twizzled_name::#lens_name <#generic_params>
                where #(#lens_where_clauses)*,
                    #ty<#generic_params>: Interactive
            {
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
            pub const #field_name: #twizzled_name::#lens_name <#generic_params> = #twizzled_name::#lens_name {
                #(#markers_init),*
            };
        }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        pub mod #twizzled_name {
            #(#lense_quotes)*
        }

        #[allow(non_upper_case_globals)]
        impl #impl_generics #ty #ty_generics #where_clause {
            #(#associated_items)*
        }
    };

    Ok(expanded)
}
