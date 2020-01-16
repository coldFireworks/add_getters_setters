#![feature(test)]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use syn::{DataStruct, DeriveInput, Field, Attribute};

enum GsType {
    Getter,
    Setter,
}

struct Gs {
    ty: GsType,
    mutable: bool,
    apply_to_all: bool
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
                    // rust does actually try the conditions in order - the complier doesn't put the least expensive first,
                    // so check the boolean variable before calling the expensive function, for optimisation.
                    // see bottom of test file for benchmarks on this
                    if self.apply_to_all || has_tag(f.attrs.iter(), token) {
                        match self.ty {
                            GsType::Getter => Some(self.gen_getter(f)),
                            GsType::Setter => Some(self.gen_setter(f)),
                        }
                    }else{
                        None
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #(#generated)*
                }
            }
        } else {
            panic!("This derive macro may not be used on enums");
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
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gs_builder = Gs {
        ty: GsType::Getter,
        mutable: false,
        apply_to_all: has_tag(ast.attrs.iter(), "get")
    };
    gs_builder.gen(&ast).into()
}

#[proc_macro_derive(AddGetterMut, attributes(get_mut))]
pub fn add_getter_mut(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gs_builder = Gs {
        ty: GsType::Getter,
        mutable: true,
        apply_to_all: has_tag(ast.attrs.iter(), "get_mut")
    };
    gs_builder.gen(&ast).into()
}

#[proc_macro_derive(AddSetter, attributes(set))]
pub fn add_setter(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gs_builder = Gs {
        ty: GsType::Setter,
        mutable: false,
        apply_to_all: has_tag(ast.attrs.iter(), "set")
    };
    gs_builder.gen(&ast).into()
}

/// Pass in an Iterator and a tag to search for in the meta data, and it will return weather the tag was found or not
fn has_tag<'a, T: Iterator<Item = &'a Attribute>>(mut attribs: T, tag_name: &str) -> bool {
    attribs
    .find_map(|v| {
        let meta = v.parse_meta().expect("failed to parse attr meta data");
        if meta.path().is_ident(tag_name) {
            Some(meta)
        } else {
            None
        }
    })
    .is_some()
}