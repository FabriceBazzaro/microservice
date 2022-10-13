use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};

use implementations::*;
mod implementations;

#[proc_macro_attribute]
pub fn injectable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes: Vec<syn::Ident> = syn::parse_macro_input!(attr with syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated)
        .into_iter()
        .collect();
    let mut ast: syn::Item = syn::parse2(TokenStream2::from(item)).expect("Failed to parse Input");

    impl_injectable(&attributes, &mut ast)
}

#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    panic!("#[inject] must be called in an impl block with the attribute #[injector]")
}

#[proc_macro_attribute]
pub fn injector(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes: Vec<syn::Ident> = syn::parse_macro_input!(attr with syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated)
        .into_iter()
        .collect();
    let mut ast: syn::Item = syn::parse2(TokenStream2::from(item)).expect("Failed to parse Input");
    impl_injector(&attributes, &mut ast)
}
