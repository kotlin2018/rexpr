#![allow(unused_must_use)]
#![allow(unused_variables)]

pub mod access;
pub mod ast;
#[macro_use]
pub mod bencher;
pub mod error;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod token;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate rexpr_macro;

pub use rexpr_macro::{expr};

//test mod
mod eval_test;
mod parser_test;
