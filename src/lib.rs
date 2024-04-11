use proc_macro::{Ident, Span, TokenStream};
use std::any::Any;
use std::ops::Deref;
use proc_macro2::extra::DelimSpan;
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{Block, Expr, ExprCall, ExprPath, ImplItemFn, ItemTrait, Meta, parse_macro_input, Path, PathArguments, PathSegment, token, Visibility};
use syn::punctuated::Punctuated;
use syn::ReturnType::Type;
use syn::Stmt::Expr;
use syn::token::{Comma, Pub};
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

fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn create_function_call(method: &proc_macro2::Ident, args: Punctuated<Expr, Comma>) -> Expr {
    let method_path = Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::from_iter(
                vec![PathSegment { ident: method.clone(), arguments: PathArguments::None }]
            ),
        },
    });

    Expr::Call(ExprCall {
        attrs: Vec::new(),
        paren_token: token::Paren { span: DelimSpan::call_site() },
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


    // let tr = input_trait.to_token_stream();
    let body = input_trait.items.iter().filter_map(|item| {
        match item {
            Fn(ti) => {
                let func = ti.clone();
                let signature = func.sig.clone();
                let r = signature.output;
                let return_type = match r {
                    Type(_, t) => type_name_of(&t),
                    _ => "Any"
                };
                let func_name = signature.ident.to_string();
                let call_expr = create_function_call(&signature.ident,);
                let ts: Option<proc_macro2::TokenStream> =
                    Some(format!("self.call::<usize>(InvocationInfo {{func_name: {}}}) as {}",
                                 func_name, return_type).parse().unwrap());
                Some(ImplItemFn {
                    attrs: func.attrs,
                    vis: Visibility::Public(Pub::default()),
                    defaultness: None,
                    sig: func.sig,
                    block: Block {
                        brace_token: token::Brace { span: DelimSpan::call_site() },
                        stmts: vec![Expr(call_expr,
                                         Some(token::Semi { spans: DelimSpan::call_site() }))],
                    },
                })
            }
            &_ => None
        }
    }).collect::<proc_macro2::TokenStream>();

    TokenStream::from(quote! {
        #inp
        impl #name for #imp {
            #body
        }
    })
}
