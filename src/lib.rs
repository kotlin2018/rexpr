#![allow(unused_must_use)]
#![allow(unused_variables)]

pub mod eval;
pub mod ast;
pub mod lexer;
pub mod runtime;
pub mod token;
pub mod parser;
pub mod access;
pub mod error;
pub mod bencher;

#[macro_use]
extern crate serde_json;

//test mod
mod parser_test;
mod eval_test;