#![recursion_limit = "128"]

extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;
// use quote::*;
extern crate proc_macro2;

pub(crate) mod attr;
mod transforms;

use proc_macro::TokenStream;
// pub(crate) use syn::Result;

#[proc_macro_attribute]
pub fn templar_filter(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item);
    transforms::impl_filter(&ast)
}

#[proc_macro_attribute]
pub fn templar_function(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item);
    transforms::impl_function(&ast)
}
