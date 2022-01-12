use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn esqtest(_stream: TokenStream, r#in: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(r#in as ItemFn);

    let body = &input.block;
    let name = &input.sig.ident;

    let full_name = quote::format_ident!("{}_esqtest", name);

    let retval = quote::quote! {
        #[test_case]
        static #full_name: crate::test::RustTest = crate::test::RustTest {
            func: #name,
            name: concat!(module_path!(), ".", stringify!(#name))
        };

        fn #name() {
            #body
        }
    };
    retval.into()
}
