use proc_macro::TokenStream;
use std::collections::hash_map::{DefaultHasher};
use std::hash::Hasher;
use quote::quote;

pub fn impl_hash(item: TokenStream) -> TokenStream {
    let mut hasher = DefaultHasher::new();
    hasher.write((&item.to_string()).as_bytes());
    let hash = hasher.finish();

    proc_macro::TokenStream::from(quote! { #hash as u64 })
}
