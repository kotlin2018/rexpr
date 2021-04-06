use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, DeriveInput, Expr, ItemFn, parse_macro_input};
use syn::parse::{Parse, ParseStream};

use crate::proc_macro::TokenStream;

pub(crate) fn impl_fn(f: &ItemFn, args: crate::proc_macro::TokenStream) -> TokenStream {
    let mut string_data = args.to_string();
    string_data = string_data[1..string_data.len() - 1].to_string();
    string_data = string_data.replace("'", "\"");
    println!("string_data:{}", string_data);
    //let s = syn::parse_str::<syn::LitStr>(&string_data).unwrap();
    let t = syn::parse_str::<Expr>(&string_data).unwrap();
    let exprs = t.to_token_stream();
    return quote!(pub fn gen(arg:&mut serde_json::Value) -> rexpr::error::Result<serde_json::Value> {
                      let v=#t;
                      println!("v:{}",v);
                     return Ok(serde_json::json!(#exprs));
                  })
        .to_token_stream().into();
}