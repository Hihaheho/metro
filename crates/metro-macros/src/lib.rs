use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

/// Derive `EntityEnum` for an enum.
///
/// ```rust
/// use metro_blackboard::prelude::*;
/// // or use metro_agent::prelude::*;
/// // or use metro_engine::prelude::*;
///
/// struct Apple;
/// struct Person { name: String }
/// struct Have;
///
/// #[derive(EntityEnum)]
/// enum Entity {
///     Apple(Apple),
///     Person(Person),
///     Have(Have),
/// }
/// ```
#[proc_macro_derive(EntityEnum)]
pub fn derive_entity_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let Data::Enum(data_enum) = &input.data else {
        return quote! {
            compile_error!("EntityEnum can only be derived for enums");
        }
        .into();
    };
    let variants = data_enum
        .variants
        .iter()
        .map(|variant| {
            let tag = variant.ident.clone();
            let syn::Fields::Unnamed(fields) = &variant.fields else {
                panic!("EntityEnum can only be derived for enums with one unnamed fields");
            };
            assert_eq!(
                fields.unnamed.len(),
                1,
                "EntityEnum can only be derived for enums with one unnamed field"
            );
            let ty = fields.unnamed[0].ty.clone();
            Variant { tag, ty }
        })
        .collect::<Vec<_>>();
    let type_tag = gen_type_tag(&input, &variants);
    let entity_enum_impl = impl_entity_enum(&input, &variants);
    let from_entity_impls = impl_from_entity(&input, &variants);
    let enum_downcast_impls = impl_enum_downcast(&input, &variants);
    let type_tag_impl = impl_type_tag(&input, &variants);
    let from_impls = impl_from(&input, &variants);

    quote! {
        #type_tag
        #entity_enum_impl
        #from_entity_impls
        #enum_downcast_impls
        #type_tag_impl
        #from_impls
    }
    .into()
}

struct Variant {
    tag: syn::Ident,
    ty: syn::Type,
}

fn gen_type_tag(input: &DeriveInput, variants: &[Variant]) -> proc_macro2::TokenStream {
    let tags = variants.iter().map(|Variant { tag, .. }| {
        quote! {
            #tag
        }
    });
    #[cfg(not(feature = "serde"))]
    let derive = quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    };
    #[cfg(feature = "serde")]
    let derive = quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
    };
    let vis = &input.vis;
    let enum_ident = &input.ident;
    let ident = format_ident!("{}TypeTag", enum_ident);
    quote! {
        #derive
        #vis enum #ident {
            #(#tags,)*
            #enum_ident,
        }
    }
}

fn impl_entity_enum(input: &DeriveInput, variants: &[Variant]) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let type_tag = format_ident!("{}TypeTag", enum_ident);
    let tags = variants.iter().map(|Variant { tag, .. }| {
        quote! {
            #tag
        }
    });
    let type_tag_match_arms = variants.iter().map(|Variant { tag, .. }| {
        quote! {
            Self::#tag(_) => #type_tag::#tag
        }
    });

    quote! {
        impl EntityEnum for #enum_ident {
            const TYPE_TAGS: &'static [Self::TypeTag] = &[
                #(Self::TypeTag::#tags,)*
                Self::TypeTag::#enum_ident,
            ];
            type TypeTag = #type_tag;
            type EntityId = DefaultEntityId;
            fn type_tag(&self) -> Self::TypeTag {
                match self {
                    #(#type_tag_match_arms,)*
                }
            }
            fn type_tag_of<T>() -> Self::TypeTag where Self: FromEntity<T>{
                <Self as FromEntity<T>>::type_tag()
            }
        }
    }
}

fn impl_from_entity(input: &DeriveInput, variants: &[Variant]) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let impls = variants.iter().map(|Variant { tag, ty }| {
        quote! {
            impl FromEntity<#ty> for #enum_ident {
                fn from_entity(entity: #ty) -> #enum_ident {
                    #enum_ident::#tag(entity)
                }
                fn type_tag() -> Self::TypeTag {
                    Self::TypeTag::#tag
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}

fn impl_enum_downcast(input: &DeriveInput, variants: &[Variant]) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let impls = variants.iter().map(|Variant { tag, ty }| {
        quote! {
            impl EnumDowncast<#enum_ident> for #ty {
                fn enum_downcast(from: #enum_ident) -> Option<Self> {
                    match from {
                        #enum_ident::#tag(entity) => Some(entity),
                        _ => None,
                    }
                }
                fn enum_downcast_ref(from: &#enum_ident) -> Option<&Self> {
                    match from {
                        #enum_ident::#tag(entity) => Some(entity),
                        _ => None,
                    }
                }
                fn enum_downcast_mut(from: &mut #enum_ident) -> Option<&mut Self> {
                    match from {
                        #enum_ident::#tag(entity) => Some(entity),
                        _ => None,
                    }
                }
            }
        }
    });
    quote! {
        #(#impls)*
    }
}

fn impl_type_tag(input: &DeriveInput, variants: &[Variant]) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let type_tag = format_ident!("{}TypeTag", enum_ident);
    let match_arms = variants.iter().map(|Variant { tag, ty }| {
        quote! {
            #type_tag::#tag => TypeInfo {
                type_id: std::any::TypeId::of::<#ty>(),
                tag_name: stringify!(#tag), // not #ty
            }
        }
    });

    quote! {
        impl TypeTag for #type_tag {
            const ANY: Self = Self::#enum_ident;
            fn type_info(&self) -> TypeInfo {
                match self {
                    #(#match_arms,)*
                    Self::#enum_ident => TypeInfo {
                        type_id: std::any::TypeId::of::<#enum_ident>(),
                        tag_name: stringify!(#enum_ident),
                    },
                }
            }
        }
    }
}

fn impl_from(input: &DeriveInput, variants: &[Variant]) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let impls = variants.iter().map(|Variant { tag, ty }| {
        quote! {
            impl From<#ty> for #enum_ident {
                fn from(entity: #ty) -> #enum_ident {
                    #enum_ident::#tag(entity)
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}
