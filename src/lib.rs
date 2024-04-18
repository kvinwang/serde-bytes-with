#![doc = include_str!("../README.md")]

extern crate proc_macro;

use syn::{parse_macro_input, punctuated::Punctuated, ItemStruct, Meta, Token};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn serde_bytes_with(attr: TokenStream, item: TokenStream) -> TokenStream {
    patch(parse_macro_input!(attr), parse_macro_input!(item)).into()
}

pub(crate) fn patch(serde_with: TokenStream2, item: ItemStruct) -> TokenStream2 {
    match patch_or_err(serde_with, item) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn has_attr(attrs: &[syn::Attribute], name: &str, field: &str) -> bool {
    for attr in attrs.iter() {
        if attr.path().is_ident(name) {
            let Ok(nested) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            else {
                continue;
            };
            for meta in nested.iter() {
                if meta.path().is_ident(field) {
                    return true;
                }
            }
        }
    }
    false
}

fn patch_or_err(serde_with: TokenStream2, mut input: ItemStruct) -> syn::Result<TokenStream2> {
    for field in input.fields.iter_mut() {
        if has_attr(&field.attrs, "serde", "with") {
            continue;
        }

        if has_attr(&field.attrs, "prost", "bytes") {
            field
                .attrs
                .push(syn::parse_quote!(#[serde(with = #serde_with)]));
        }
    }

    Ok(syn::parse_quote! {
        #input
    })
}

#[cfg(test)]
mod tests;
