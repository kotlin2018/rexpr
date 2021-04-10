use quote::quote;
use quote::ToTokens;
use syn;
use syn::{Expr, ItemFn, ExprPath};

use crate::proc_macro::TokenStream;
use std::any::Any;
use syn::spanned::Spanned;

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
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str fail for: {}", t.err().unwrap().to_string())
    } else {
        println!("[rexpr]parse expr:{} success!", string_data);
    }
    let mut t = t.unwrap();
    t = convert_to_arg_access(t);
    string_data = t.to_token_stream().to_string();
    string_data = string_data.replace("arg", "@").replace(" . ", ".");
    println!("[rexpr]expr:{}", string_data);

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

    //let s = syn::parse_str::<syn::LitStr>(&string_data).unwrap();
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str fail for: {}", t.err().unwrap().to_string())
    } else {
        println!("[rexpr]parse expr:{} success!", string_data);
    }
    let mut t = t.unwrap();
    //t = convert_to_arg_access(t);

    println!("[rexpr]gen expr: {}", t.to_token_stream());
    let func_args = f.sig.inputs.to_token_stream();
    let func_name_ident = f.sig.ident.to_token_stream();
    let mut return_ty = f.sig.output.to_token_stream();
    return quote!(pub fn #func_name_ident(#func_args)  #return_ty {
                     let result=#t;
                     if result.eq(&serde_json::Value::Null){
                      return Ok(serde_json::Value::Null);
                     }
                     return Ok(serde_json::json!(result));
                  })
        .to_token_stream().into();
}

fn convert_to_arg_access(arg: Expr) -> Expr {
    match arg {
        Expr::Path(b) => {
            if b.to_token_stream().to_string().trim() == "null" {
                return syn::parse_str::<Expr>("serde_json::Value::Null").unwrap();
            }
            println!("[rexpr]Path:{}", b.to_token_stream());
            return syn::parse_str::<Expr>(&format!("arg.{}", b.to_token_stream())).unwrap();
        }
        Expr::MethodCall(b) => {
            // println!("[rexpr]MethodCall:{}", &b.receiver);
            match *b.receiver.clone() {
                Expr::Path(pp) => {
                    println!("[rexpr]MethodCall:{}", pp.to_token_stream());
                    //return syn::parse_str::<Expr>(&format!("arg.{}", b.to_token_stream())).unwrap();
                }
                _ => {}
            }
            return syn::parse_str::<Expr>(&format!("arg.{}", b.to_token_stream())).unwrap();
        }
        Expr::Binary(mut b) => {
            //println!("[rexpr]Binary:{}", b.to_token_stream());
            //println!("[rexpr]BinaryLeft:{}", b.left.to_token_stream());
            //println!("[rexpr]Binary:{}", b.right.to_token_stream());
            b.left = Box::new(convert_to_arg_access(*b.left.clone()));
            b.right = Box::new(convert_to_arg_access(*b.right.clone()));
            return Expr::Binary(b);
        }
        Expr::Unary(mut b) => {
            b.expr = Box::new(convert_to_arg_access(*b.expr.clone()));
            return Expr::Unary(b);
        }
        _ => {
            return arg;
        }
    }
}