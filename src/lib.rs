#![allow(unused_must_use)]
#![allow(unused_variables)]

pub mod access;
pub mod ast;
pub mod bencher;
pub mod error;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod token;
mod cache;

#[macro_use]
extern crate serde_json;

//test mod
mod eval_test;
mod parser_test;
