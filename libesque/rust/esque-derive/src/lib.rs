use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn main(_stream: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);

    let body = &input.block;

    let retval = quote::quote! {
        extern crate esque;

        #[no_mangle]
        pub extern "C" fn main() -> u32 {
            fn entry() -> u32 {
                #body
            };
            return entry();
        }
    };
    retval.into()
}
