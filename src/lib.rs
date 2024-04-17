use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, ExprCall, ExprPath, ImplItemFn, ItemTrait, Meta, parse_macro_input, Path, PathArguments, PathSegment, Stmt, token, Type};
use syn::punctuated::Punctuated;
use core::default::Default;
use std::ops::Deref;
use std::time::SystemTime;
use syn::token::{Brace, Comma};
use syn::TraitItem::Fn;
use syn::Visibility::Inherited;
use log::{info, warn};
use proc_macro2::{Ident, Span};
// use dynamic_proxy_types::{DynamicProxy, InvocationInfo};

extern crate proc_macro;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}


fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn create_function_call(method: &proc_macro2::Ident, args: Punctuated<syn::Expr, Comma>) -> syn::Expr {
    let method_path = syn::Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::from_iter(
                vec![PathSegment { ident: method.clone(), arguments: PathArguments::None }]
            ),
        },
    });

    syn::Expr::Call(ExprCall {
        attrs: Vec::new(),
        paren_token: token::Paren::default(),
        func: Box::new(method_path),
        args: args.clone(),
    })
}

#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn dynamic_proxy(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    let input_struct =
        parse_macro_input!(_metadata with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let input_trait = parse_macro_input!(_input as ItemTrait);
    let inp = input_trait.clone();
    let name = input_trait.ident;
    let imp = input_struct.first();
    let _ = setup_logger();

    // let tr = input_trait.to_token_stream();
    let body = input_trait.items.iter().filter_map(|item| {
        match item {
            Fn(ti) => {
                let func = ti.clone();
                let signature = func.sig.clone();
                let func_name = signature.ident.to_string();
                let r = signature.output;
                let return_type = match r {
                    syn::ReturnType::Type(_, t) => t.deref().to_token_stream(),
                    _ => quote!(Any)
                };
                let stmt: Vec<Stmt> = syn::parse_quote! (
                    let mut invocation_info = InvocationInfo {
                        func_name: #func_name,
                        return_value: None};
                    self.call(&mut invocation_info);
                    return invocation_info.return_value.unwrap().downcast::<#return_type>().unwrap().deref().clone();
                );
                Some(ImplItemFn {
                    attrs: func.attrs,
                    vis: Inherited,
                    defaultness: None,
                    sig: func.sig,
                    block: Block {
                        brace_token: Brace::default(),
                        stmts: stmt,
                    },
                })
            }
            &_ => None
        }
    });
    
    let res = TokenStream::from(quote! {
        #inp
        impl #name for #imp {
            #(#body)*
        }
    });
    
    warn!("syn {}", res);
    res
}
