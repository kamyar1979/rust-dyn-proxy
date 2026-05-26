use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, ImplItem, ImplItemFn, ItemTrait, Meta, parse_macro_input, Pat, Stmt, TraitItem, Type};
use syn::punctuated::Punctuated;
use core::default::Default;
use std::ops::Deref;
use syn::token::Brace;
use syn::Visibility::Inherited;
use syn::FnArg::Typed;

extern crate proc_macro;

#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn dynamic_proxy(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    let input_struct =
        parse_macro_input!(_metadata with Punctuated::<Meta, syn::Token![,]>::parse_terminated);
    let input_trait = parse_macro_input!(_input as ItemTrait);
    let inp = &input_trait;
    let name = &input_trait.ident;
    let (impl_generics, ty_generics, where_clause) = input_trait.generics.split_for_impl();
    let imp = input_struct.first();

    // let tr = input_trait.to_token_stream();
    let body = input_trait.items.iter().filter_map(|item| {
        match item {
            TraitItem::Fn(ti) => {
                let func = ti.to_owned();
                let signature = func.sig;
                let is_async = signature.asyncness.is_some();
                let func_name = signature.ident.to_string();
                let mut errors = Vec::new();
                let args: Vec<_> = signature.inputs.iter().filter_map(|a| {
                    match a {
                        Typed(t) => match t.clone().pat.deref() {
                            Pat::Ident(id) => {
                                if matches!(t.ty.deref(), Type::Reference(_)) {
                                    errors.push(syn::Error::new_spanned(
                                        &t.ty,
                                        "dynamic_proxy does not support borrowed arguments; use owned 'static + Send + Sync arguments",
                                    ).to_compile_error());
                                }
                                Some(id.clone().ident)
                            },
                            _ => {
                                errors.push(syn::Error::new_spanned(
                                    &t.pat,
                                    "dynamic_proxy only supports identifier arguments",
                                ).to_compile_error());
                                None
                            }
                        },
                        _ => None
                    }
                }).collect();
                let arg_names = args.iter().map(|i| i.to_string());
                let r = &signature.output;
                let (return_type, returns_unit) = match r {
                    syn::ReturnType::Type(_, t) => {
                        if matches!(t.deref(), Type::Reference(_)) {
                            errors.push(syn::Error::new_spanned(
                                t,
                                "dynamic_proxy does not support borrowed return values; use owned 'static + Send + Sync return values",
                            ).to_compile_error());
                        }
                        (t.deref().to_token_stream(), false)
                    },
                    _ => (quote!(()), true)
                };
                let invocation_stmt: Vec<Stmt> = syn::parse_quote!(
                                        let mut invocation_info = ::dynamic_proxy::InvocationInfo {
                        func_name: #func_name,
                        arg_names: &[#(#arg_names),*],
                        arg_values: &[#(::std::boxed::Box::new(#args)),*],
                        return_type: ::std::any::TypeId::of::<#return_type>(),
                        return_value: None
                    };
                );
                
                let call_stmt: Vec<Stmt> =
                    match (is_async, returns_unit)
                    {
                        (true, true) => syn::parse_quote!(
                    ::dynamic_proxy::AsyncDynamicProxy::call_async(&self, &mut invocation_info).await;
                        ),
                        (true, false) => syn::parse_quote!(
                    ::dynamic_proxy::AsyncDynamicProxy::call_async(&self, &mut invocation_info).await;
                    return *invocation_info.return_value.unwrap().downcast::<#return_type>().unwrap();
                        ),
                        (false, true) => syn::parse_quote!(
                    ::dynamic_proxy::DynamicProxy::call(&self, &mut invocation_info);
                        ),
                        (false, false) => syn::parse_quote!(
                    ::dynamic_proxy::DynamicProxy::call(&self, &mut invocation_info);
                    return *invocation_info.return_value.unwrap().downcast::<#return_type>().unwrap();
                        )
                    };
                
                let error_stmt: Vec<Stmt> = errors.into_iter()
                    .map(|error| syn::parse_quote!(#error))
                    .collect();
                let stmt = [error_stmt.as_slice(), invocation_stmt.as_slice(), call_stmt.as_slice()].concat();
                
                Some(ImplItem::Fn(ImplItemFn {
                    attrs: func.attrs,
                    vis: Inherited,
                    defaultness: None,
                    sig: signature,
                    block: Block {
                        brace_token: Brace::default(),
                        stmts: stmt,
                    },
                }))
            }
            TraitItem::Const(item) if item.default.is_none() => {
                Some(syn::parse_quote! {
                    compile_error!("dynamic_proxy does not support required associated constants");
                })
            }
            TraitItem::Type(item) if item.default.is_none() => {
                Some(syn::parse_quote! {
                    compile_error!("dynamic_proxy does not support required associated types");
                })
            }
            &_ => None
        }
    });

    TokenStream::from(quote! {
        #inp
        impl #impl_generics #name #ty_generics for #imp #where_clause {
            #(#body)*
        }
    })
}
