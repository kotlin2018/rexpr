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
    string_data = string_data.replace("null", "serde_json::Value::Null");
    //as
    string_data = string_data.replace(".as_i32()",".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".as_i64()",".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".as_f64()",".as_f64().unwrap_or(0.0)");
    string_data = string_data.replace(".as_str()",".as_str().unwrap_or(\"\")");

    string_data = string_data.replace(".i32()",".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".i64()",".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".f64()",".as_f64().unwrap_or(0.0)");
    string_data = string_data.replace(".str()",".as_str().unwrap_or(\"\")");


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