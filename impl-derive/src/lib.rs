// impl-derive/src/lib.rs
extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

// 为某个类型实现MyTrait
fn impl_mytrait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MyTrait for #name {
            fn do_something() {
                println!("Do something, my name is {}", stringify!(#name));
            }
        }
    };

    gen.into()
}

// 把宏和函数对应起来，如果类型上面加了#[derive(MyDeriveMacro)]
// 就类似于是为其调用impl_mytrait_derive函数
#[proc_macro_derive(MyDeriveMacro)]
pub fn impl_mytrait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); //DeriveInput
    impl_mytrait(&ast)
}