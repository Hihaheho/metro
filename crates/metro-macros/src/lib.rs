use quote::{format_ident, quote};
use syn::{Data, DataEnum, DeriveInput};

#[proc_macro_derive(EntityEnum)]
pub fn derive_entity_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let Data::Enum(data_enum) = &input.data else {
        return quote! {
            compile_error!("EntityEnum can only be derived for enums");
        }
        .into();
    };
    let new_enum = gen_enum(&input, data_enum);
    let type_tag = gen_type_tag(&input, data_enum);
    let entity_enum_impl = impl_entity_enum(&input, data_enum);
    let from_entity_impls = impl_from_entity(&input, data_enum);
    let enum_downcast_impls = impl_enum_downcast(&input, data_enum);
    let type_tag_impl = impl_type_tag(&input, data_enum);
    let from_impls = impl_from(&input, data_enum);

    quote! {
        #new_enum
        #type_tag
        #entity_enum_impl
        #from_entity_impls
        #enum_downcast_impls
        #type_tag_impl
        #from_impls
    }
    .into()
}

fn gen_enum(input: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter().map(|variant| {
        let ident = variant.ident.clone();
        quote! {
            #ident(#ident)
        }
    });
    let attrs = &input.attrs;
    let vis = &input.vis;
    let ident = &input.ident;
    quote! {
        #(#attrs)*
        #vis enum #ident {
            #(#variants,)*
        }
    }
}

fn gen_type_tag(input: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let variants = data_enum.variants.iter();
    #[cfg(not(feature = "serde"))]
    let derive = quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    };
    #[cfg(feature = "serde")]
    let derive = quote! {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
    };
    let vis = &input.vis;
    let ident = format_ident!("{}TypeTag", input.ident);
    quote! {
        #derive
        #vis enum #ident {
            #(#variants,)*
        }
    }
}

fn impl_entity_enum(input: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let type_tag = format_ident!("{}TypeTag", ident);
    let variants = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            #ident
        }
    });
    let match_arms = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            #ident::#ident(_) => #type_tag::#ident
        }
    });

    quote! {
        impl sj_blackboard::entity_traits::EntityEnum for #ident {
            type TypeTag = #type_tag;
            fn get_type_tag(&self) -> Self::TypeTag {
                match self {
                    #(#match_arms,)*
                }
            }
            fn list_type_tags() -> &'static [Self::TypeTag] {
                static TYPE_TAGS: &[Self::TypeTag] = &[
                    #(Self::TypeTag::#variants,)*
                ]
            }
        }
    }
}

fn impl_from_entity(input: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let type_tag = format_ident!("{}TypeTag", enum_ident);
    let impls = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            impl sj_blackboard::entity_traits::FromEntity<#ident> for #enum_ident {
                fn from_entity(entity: #ident) -> #enum_ident {
                    #enum_ident::#ident(entity)
                }
                fn type_tag() -> #type_tag {
                    #type_tag::#ident
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}

fn impl_enum_downcast(input: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let type_tag = format_ident!("{}TypeTag", enum_ident);
    let impls = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            impl sj_blackboard::entity_traits::EnumDowncast<#ident> for #enum_ident {
                fn enum_downcast(self) -> #ident {
                    let #enum_ident::#ident(entity) = self else {
                        panic!("enum_downcast called on wrong variant. {:?} to {:?}", self.type_tag(), #type_tag::#ident);
                    }
                    entity
                }
                fn enum_downcast_ref(&self) -> &#ident {
                    let #enum_ident::#ident(entity) = self else {
                        panic!("enum_downcast_ref called on wrong variant. {:?} to {:?}", self.type_tag(), #type_tag::#ident);
                    }
                    entity
                }
                fn enum_downcast_mut(&mut self) -> &mut #ident {
                    let #enum_ident::#ident(entity) = self else {
                        panic!("enum_downcast_mut called on wrong variant. {:?} to {:?}", self.type_tag(), #type_tag::#ident);
                    }
                    entity
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}

fn impl_type_tag(input: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let type_tag = format_ident!("{}TypeTag", input.ident);
    let match_arms = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let name = ident.to_string();
        quote! {
            #type_tag::#ident => sj_blackboard::entity_traits::TypeInfo {
                type_id: std::any::TypeId::of::<#ident>(),
                name: #name,
            }
        }
    });

    quote! {
        impl sj_blackboard::entity_traits::TypeTag for #type_tag {
            fn type_info(&self) -> sj_blackboard::type_info::TypeInfo {
                match self {
                    #(#match_arms,)*
                }
            }
        }
    }
}

fn impl_from(input: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let enum_ident = &input.ident;
    let impls = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            impl From<#ident> for #enum_ident {
                fn from(entity: #ident) -> #enum_ident {
                    #enum_ident::#ident(entity)
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}
