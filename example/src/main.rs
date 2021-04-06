#[macro_use]
extern crate rexpr;

use rexpr::runtime::RExprRuntime;


fn main() {
    let runtime = RExprRuntime::new();
    let result = runtime.eval("1+1", &serde_json::json!(null)).unwrap();
    println!("result: {}", result);
}

#[expr("1+2")]
pub fn gen() -> rexpr::error::Result<serde_json::Value>{}

#[test]
fn it_works() {
    let v = gen();
    println!("{}", v.unwrap());
}
