
use proc_macro::TokenStream;
use std::any::Any;
use std::ops::Deref;
use quote::{quote, ToTokens};
use syn::{ItemStruct, ItemTrait, parse_macro_input, TraitItemFn};
use syn::FnArg::{Receiver, Typed};
use syn::Pat::Lit;
use syn::TraitItem::Fn;
use dynamic_proxy_types::{DynamicProxy, InvocationInfo};

extern crate proc_macro;


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn dynamic_proxy(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(_input as ItemStruct);
    let input_trait = parse_macro_input!(_metadata as ItemTrait);
    let name = input_trait.ident;
    let imp = input_struct.ident;
    let body = input_trait.items.iter().map(|item| {
        match item {
            Fn(func) => |func: TraitItemFn| {
                let signature = func.sig;
                let func_name = signature.ident;
                let args = signature.inputs;
                func.to_token_stream();
                quote! {

                }
            },
            &_ => todo!()
        }
    });
    TokenStream::from(quote!{
        impl #name for #imp {
        }
    })
}
