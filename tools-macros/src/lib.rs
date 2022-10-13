use proc_macro::TokenStream;
use implementations::impl_hash;
mod implementations;

/// Macro returning the hash value for a string into an u64
#[proc_macro]
pub fn hash(item: TokenStream) -> TokenStream {
    impl_hash(item)
}
