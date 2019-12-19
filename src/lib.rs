extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use syn::{DataStruct, DeriveInput, Field};

enum GsType {
    Getter,
    Setter,
}

struct Gs {
    ty: GsType,
    mutable: bool
}

impl Gs {
    pub fn gen(self, ast: &DeriveInput) -> TokenStream2 {
        let name = &ast.ident;
        let generics = &ast.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
            let generated = fields
                .iter()
                .filter_map(|f| {
                    let token = match self.ty {
                        GsType::Getter if self.mutable => "get_mut",
                        GsType::Getter => "get",
                        GsType::Setter => "set",
                    };
                    match f.attrs.iter().find_map(|v| {
                        match v.parse_meta() {
                            Ok(md) => {
                                if md.path().is_ident(token) {
                                    Some(md)
                                } else {
                                    None
                                }
                            },
                            _ => None
                        }
                    }) {
                        Some(_) => {
                            match self.ty {
                                GsType::Getter => Some(self.gen_getter(f)),
                                GsType::Setter => Some(self.gen_setter(f)),
                            }
                        },
                        None => None,
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #(#generated)*
                }
            }
        } else {
            panic!("#[derive(Getters)] is only defined for structs, not for enums!");
        }
    }

    fn gen_setter(&self, field: &Field) -> TokenStream2 {
        let field_name = field.clone().ident.unwrap();
        let fn_name = Ident::new(&format!("set_{}", field_name), Span::call_site());
        let ty = field.ty.clone();
        quote! {
            #[inline(always)]
            pub fn #fn_name(&mut self, v: #ty) {
                self.#field_name = v;
            }
        }
    }

    fn gen_getter(&self, field: &Field) -> TokenStream2 {
        let field_name = field.clone().ident.unwrap();
        let ty = field.ty.clone();
        let fn_name = Ident::new(&format!("get_{}{}", field_name, if self.mutable{ "_mut" } else { "" }), Span::call_site());
        if self.mutable {
            quote! {
                #[inline(always)]
                pub fn #fn_name(&mut self) -> &mut #ty {
                    &mut self.#field_name
                }
            }
        } else {
            quote! {
                #[inline(always)]
                pub fn #fn_name(&self) -> &#ty {
                    &self.#field_name
                }
            }
        }
    }
}

#[proc_macro_derive(AddGetter, attributes(get))]
pub fn add_getter(input: TokenStream) -> TokenStream {
    let gs_builder = Gs {
        ty: GsType::Getter,
        mutable: false
    };
    gs_builder.gen(&syn::parse(input).unwrap()).into()
}

#[proc_macro_derive(AddGetterMut, attributes(get_mut))]
pub fn add_getter_mut(input: TokenStream) -> TokenStream {
    let gs_builder = Gs {
        ty: GsType::Getter,
        mutable: true
    };
    gs_builder.gen(&syn::parse(input).unwrap()).into()
}

#[proc_macro_derive(AddSetter, attributes(set))]
pub fn add_setter(input: TokenStream) -> TokenStream {
    let gs_builder = Gs {
        ty: GsType::Setter,
        mutable: false
    };
    gs_builder.gen(&syn::parse(input).unwrap()).into()
}