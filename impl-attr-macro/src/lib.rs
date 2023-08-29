use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// 定义属性宏，属性宏的名字就叫做func_info
#[proc_macro_attribute]
pub fn func_info(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);
    let func_name = &func.sig.ident;
    let func_block = &func.block;
    let output = quote! {
        {
            println!("fun {} starts", stringify!(#func_name));
            let __log_result = { #func_block };
            println!("fun {} ends", stringify!(#func_name));
            __log_result
        }
    };
    func.block = syn::parse2(output).unwrap();
    quote! { #func }.into()
}
