use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, ImplItemFn, ItemTrait, Meta, parse_macro_input, Pat, Stmt};
use syn::punctuated::Punctuated;
use core::default::Default;
use std::ops::Deref;
use std::time::SystemTime;
use syn::token::Brace;
use syn::TraitItem::Fn;
use syn::Visibility::Inherited;
use syn::FnArg::Typed;

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

#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn dynamic_proxy(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    let input_struct =
        parse_macro_input!(_metadata with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let input_trait = parse_macro_input!(_input as ItemTrait);
    let inp = &input_trait;
    let name = &input_trait.ident;
    let imp = input_struct.first();
    let _ = setup_logger();

    // let tr = input_trait.to_token_stream();
    let body = input_trait.items.iter().filter_map(|item| {
        match item {
            Fn(ti) => {
                let func = ti.to_owned();
                let signature = func.sig;
                let is_async = signature.asyncness.is_some();
                let func_name = signature.ident.to_string();
                let args = signature.inputs.iter().filter_map(|a| {
                    match a {
                        Typed(t) => match t.clone().pat.deref() {
                            Pat::Ident(id) => Some(id.clone().ident),
                            _ => None
                        },
                        _ => None
                    }
                });
                let arg_names = args.clone().map(|i| i.to_string());
                let r = &signature.output;
                let return_type = match r {
                    syn::ReturnType::Type(_, t) => t.deref().to_token_stream(),
                    _ => quote!(Any)
                };
                let invocation_stmt: Vec<Stmt> = syn::parse_quote!(
                                        let mut invocation_info = InvocationInfo {
                        func_name: #func_name,
                        arg_names: &[#(#arg_names),*],
                        arg_values: &[#(Box::new(#args)),*],
                        return_type: TypeId::of::<#return_type>(),
                        return_value: None
                    };
                );
                
                let call_stmt: Vec<Stmt> =
                    match is_async
                    {
                        true => syn::parse_quote!(
                    self.call_async(&mut invocation_info).await;
                    return invocation_info.return_value.unwrap().downcast::<#return_type>().unwrap().deref().clone();
                        ),
                        _ => syn::parse_quote!(
                    self.call(&mut invocation_info);
                    return invocation_info.return_value.unwrap().downcast::<#return_type>().unwrap().deref().clone();
                        )
                    };
                
                let stmt = [invocation_stmt.as_slice(), call_stmt.as_slice()].concat();
                
                Some(ImplItemFn {
                    attrs: func.attrs,
                    vis: Inherited,
                    defaultness: None,
                    sig: signature,
                    block: Block {
                        brace_token: Brace::default(),
                        stmts: stmt,
                    },
                })
            }
            &_ => None
        }
    });

    TokenStream::from(quote! {
        #inp
        impl #name for #imp {
            #(#body)*
        }
    })
}
