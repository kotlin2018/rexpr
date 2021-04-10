use quote::quote;
use quote::ToTokens;
use syn;
use syn::{Expr, ItemFn, ExprPath, Member};

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

fn is_param_char(arg: char) -> bool {
    match arg {
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
    string_data = string_data.replace(".string()", ".to_string()");
    string_data = string_data.replace("'", "\"");

    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str fail for: {}", t.err().unwrap().to_string())
    } else {
        println!("[rexpr]parse expr:{} success!", string_data);
    }
    let mut t = t.unwrap();
    t = convert_to_arg_access(t);
    string_data = t.to_token_stream().to_string();
    string_data = string_data.replace("expr_arg", "@").replace(" . ", ".");
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
            return syn::parse_str::<Expr>(&format!("expr_arg.{}", b.to_token_stream())).unwrap();
        }
        Expr::MethodCall(b) => {
            let ex = *(b.receiver.clone());
            let s = ex.to_token_stream().to_string();
            for x in s.chars() {
                if is_param_char(x) {
                    return syn::parse_str::<Expr>(&format!("expr_arg.{}", b.to_token_stream())).unwrap();
                }
                break;
            }
            return Expr::MethodCall(b);
        }
        Expr::Binary(mut b) => {
            b.left = Box::new(convert_to_arg_access(*b.left.clone()));
            b.right = Box::new(convert_to_arg_access(*b.right.clone()));
            return Expr::Binary(b);
        }
        Expr::Unary(mut b) => {
            b.expr = Box::new(convert_to_arg_access(*b.expr.clone()));
            return Expr::Unary(b);
        }
        Expr::Field(b) => {
            return match b.member.clone() {
                Member::Named(named) => {
                    return syn::parse_str::<Expr>(&format!("expr_arg.{}", b.to_token_stream())).unwrap();
                }
                Member::Unnamed(unamed) => {
                    return Expr::Field(b);
                }
            };
        }
        Expr::Reference(mut b)=>{
            b.expr = Box::new(convert_to_arg_access(*b.expr.clone()));
            return Expr::Reference(b);
        }
        _ => {
            return arg;
        }
    }
}

fn expr_type(expr: Expr) -> String {
    match expr {
        Expr::Array(_) => { format!("Array") }
        Expr::Assign(_) => { format!("Assign") }
        Expr::AssignOp(_) => { format!("AssignOp") }
        Expr::Async(_) => { format!("Async") }
        Expr::Await(_) => { format!("Await") }
        Expr::Binary(_) => { format!("Binary") }
        Expr::Block(_) => { format!("Block") }
        Expr::Box(_) => { format!("Box") }
        Expr::Break(_) => { format!("Break") }
        Expr::Call(_) => { format!("Call") }
        Expr::Cast(_) => { format!("Cast") }
        Expr::Closure(_) => { format!("Closure") }
        Expr::Continue(_) => { format!("Continue") }
        Expr::Field(_) => { format!("Field") }
        Expr::ForLoop(_) => { format!("ForLoop") }
        Expr::Group(_) => { format!("Group") }
        Expr::If(_) => { format!("If") }
        Expr::Index(_) => { format!("Index") }
        Expr::Let(_) => { format!("Let") }
        Expr::Lit(_) => { format!("Lit") }
        Expr::Loop(_) => { format!("Loop") }
        Expr::Macro(_) => { format!("Macro") }
        Expr::Match(_) => { format!("Match") }
        Expr::MethodCall(_) => { format!("MethodCall") }
        Expr::Paren(_) => { format!("Paren") }
        Expr::Path(_) => { format!("Path") }
        Expr::Range(_) => { format!("Range") }
        Expr::Reference(_) => { format!("Reference") }
        Expr::Repeat(_) => { format!("Repeat") }
        Expr::Return(_) => { format!("Return") }
        Expr::Struct(_) => { format!("Struct") }
        Expr::Try(_) => { format!("Try") }
        Expr::TryBlock(_) => { format!("TryBlock") }
        Expr::Tuple(_) => { format!("Tuple") }
        Expr::Type(_) => { format!("Type") }
        Expr::Unary(_) => { format!("Unary") }
        Expr::Unsafe(_) => { format!("Unsafe") }
        Expr::Verbatim(_) => { format!("Verbatim") }
        Expr::While(_) => { format!("While") }
        Expr::Yield(_) => { format!("Yield") }
        Expr::__TestExhaustive(_) => { format!("__TestExhaustive") }
    }
}