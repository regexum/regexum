//! # regexum-macros
//!
//! Check [`regexum`](https://docs.rs/regexum) instead.
//!

extern crate proc_macro;

mod attrs;
use attrs::{Attr, HasAttrs};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Field, Ident, LitStr};

fn derive_patternize(ast: &DeriveInput) -> syn::Result<TokenStream> {
    use syn::{Data, DataEnum};
    
    let ident = &ast.ident;
    match ast.data {
        Data::Enum(DataEnum { ref variants, .. }) => {
            let mut patterns: Vec<LitStr> = Vec::new();
            for variant in variants {
                let attrs = variant.parse_attributes()?;
                let pattern_attr = attrs
                    .iter()
                    .filter_map(|attr| match attr {
                        Attr::Pattern(pattern) => Some(pattern),
                        _ => None,
                    })
                    .next();
                if let Some(attr) = pattern_attr {
                    let pattern = attr.pattern();
                    patterns.push(LitStr::new(pattern, variant.span()));
                }
            }
            Ok(quote! {
                impl ::regexum::Patternize for #ident {
                    fn patterns() -> &'static [&'static str] {
                        &[#(#patterns),*]
                    }
                    fn from_captures<'c>(index: usize, captures: ::regexum::Captures<'c>) -> ::regexum::Result<Self> {
                        unimplemented!()
                    }
                }
            })
        }
        _ => Err(syn::Error::new(
            Span::call_site(),
            "only supports enum variants",
        )),
    }
}

#[proc_macro_derive(Patternize, attributes(regexum, pattern))]
pub fn patternize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    derive_patternize(&ast)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
