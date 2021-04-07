use proc_macro2::{Ident, Span};
use quote::quote;
use quote::ToTokens;
use syn;
use syn::{AttributeArgs, DeriveInput, Expr, ItemFn, parse_macro_input};
use syn::parse::{Parse, ParseStream};

use crate::proc_macro::TokenStream;

fn is_name_char(arg: char) -> bool {
    match arg {
        '.' |
        '_' |
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
        'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z'
        => {
            return true;
        }
        _ => {}
    }
    return false;
}

pub(crate) fn impl_fn(f: &ItemFn, args: crate::proc_macro::TokenStream) -> TokenStream {
    let mut string_data = args.to_string();
    string_data = string_data[1..string_data.len() - 1].to_string();
    string_data = string_data.replace("null", "serde_json::Value::Null");


    //access field convert
    let mut ats = vec![];
    let mut at = false;
    let mut string_start = false;
    let mut last = None;
    for x in string_data.chars() {
        if last != Some('\\') && (x == '\'' || x == '"') {
            if string_start {
                string_start = false;
            } else {
                string_start = true;
            }
            last = Some(x);
            continue;
        }
        if string_start == false {
            if x == '@' {
                at = true;
                ats.push(x.to_string());
                continue;
            }
            if at {
                if is_name_char(x) || x == '(' || x == ')' {
                    ats.last_mut().unwrap().push(x);
                } else {
                    at = false;
                }
            }
        }
    }
    for at in ats {
        let mut new_at = String::new();

        let items: Vec<&str> = at.split(".").collect();
        let mut at_start = false;
        for x in items {
            if x == "@" {
                new_at.push_str("arg");
                at_start = true;
            } else if x.ends_with("()") {
                //method
                at_start = false;
                new_at.push_str(".");
                new_at.push_str(x);
            } else {
                if at_start {
                    new_at.push_str("[\"");
                    new_at.push_str(x);
                    new_at.push_str("\"]");
                }
            }
        }
        string_data = string_data.replace(&at, &new_at);
    }


    //remove string escape
    let mut last = None;
    let mut new_data = String::new();
    for x in string_data.chars() {
        if last.ne(&Some('\\')) && (x == '\'' || x == '"') {
            if string_start {
                string_start = false;
            } else {
                string_start = true;
            }
            last = Some(x);
            new_data.push('\"');
            continue;
        }
        new_data.push(x);
        last = Some(x);
        continue;
    }
    string_data = new_data;
    //as
    string_data = string_data.replace(".as_i32()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".as_i64()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".as_f64()", ".as_f64().unwrap_or(0.0)");
    string_data = string_data.replace(".as_str()", ".as_str().unwrap_or(\"\")");
    string_data = string_data.replace(".as_bool()", ".as_bool().unwrap_or(false)");

    string_data = string_data.replace(".i32()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".i64()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".f64()", ".as_f64().unwrap_or(0.0)");
    string_data = string_data.replace(".str()", ".as_str().unwrap_or(\"\")");
    string_data = string_data.replace(".bool()", ".as_bool().unwrap_or(false)");
    string_data = string_data.replace(".string()", ".as_str().unwrap_or(\"\").to_string()");

    println!("string_data:{}", string_data);
    //let s = syn::parse_str::<syn::LitStr>(&string_data).unwrap();
    let t = syn::parse_str::<Expr>(&string_data).unwrap();
    let exprs = t.to_token_stream();
    return quote!(pub fn gen(arg:&serde_json::Value) -> rexpr::error::Result<serde_json::Value> {
                      let v=#t;
                     return Ok(serde_json::json!(#exprs));
                  })
        .to_token_stream().into();
}