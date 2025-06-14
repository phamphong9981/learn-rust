use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, DeriveInput, LitStr, Token};

// Cấu trúc để parse input của macro
struct GreetingInput {
    name: LitStr,
    _comma: Token![,],
    age: LitStr,
}

// Implement Parse trait cho GreetingInput
impl Parse for GreetingInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GreetingInput {
            name: input.parse()?,
            _comma: input.parse()?,
            age: input.parse()?,
        })
    }
}

// Một function macro đơn giản
#[proc_macro]
pub fn make_greeting(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let expanded = quote! {
        println!("Xin chào, {}!", #input);
    };
    expanded.into()
}

#[proc_macro]
pub fn make_greeting2(input: TokenStream) -> TokenStream {
    let GreetingInput { name, age, .. } = parse_macro_input!(input as GreetingInput);
    let expanded = quote! {
        println!("Xin chào, {}! Bạn {} tuổi.", #name, #age);
    };
    expanded.into()
}

// Một derive macro
#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub fn hello_world(&self) {
                println!("Xin chào từ {}! {}", stringify!(#name), self.name);
            }
        }
    };
    expanded.into()
}

// Một attribute macro
#[proc_macro_attribute]
pub fn log_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;

    let expanded = quote! {
        fn #fn_name() {
            println!("Bắt đầu hàm {}", stringify!(#fn_name));
            #fn_body
            println!("Kết thúc hàm {}", stringify!(#fn_name));
        }
    };
    expanded.into()
}
