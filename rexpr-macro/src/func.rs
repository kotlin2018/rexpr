use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, ItemFn, Expr, DeriveInput, parse_macro_input};

use crate::proc_macro::TokenStream;
use syn::parse::{ParseStream, Parse};

pub(crate) fn impl_fn(f: &ItemFn, args: crate::proc_macro::TokenStream) -> TokenStream {
    let s = syn::parse::<syn::LitStr>(args).unwrap();
    let t = syn::parse_str::<Expr>(&s.value()).unwrap();
    //println!("expr:{}", s.as_str());
    let exprs = t.to_token_stream();
    return quote!(pub fn gen(arg:&mut serde_json::Value) -> rexpr::error::Result<serde_json::Value> {
                      let v=#t;
                      println!("v:{}",v);
                     return Ok(serde_json::json!(#exprs));
                  })
        .to_token_stream().into();
}