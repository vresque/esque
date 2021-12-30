#![no_std]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(_ /* No Attributes */: TokenStream, item: TokenStream) -> TokenStream {
    item
}
