use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn test(_stream: TokenStream, r#in: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(r#in as ItemFn);

    let body = &input.block;
    let name = &input.sig.ident;

    let full_name = quote::format_ident!("{}_esqtest", name);

    let retval = quote::quote! {
        fn #name() -> i32 {
            #body
        }

        #[test_case]
        #[allow(non_upper_case_globals)]
        static #full_name: esqtest::RustTest = esqtest::RustTest {
            func: #name,
            name: concat!(module_path!(), ".", stringify!(#name))
        };
    };
    retval.into()
}
