#[macro_use]
extern crate rexpr;

use rexpr::runtime::RExprRuntime;

fn main() {
    let runtime = RExprRuntime::new();
    let result = runtime.eval("1+1", &serde_json::json!(null)).unwrap();
    println!("result: {}", result);
}

macro_rules! attr {
    () => {};
}

#[expr("&arg['a']['b'].as_i64() != &null")]
pub fn gen(arg: &mut serde_json::Value) -> rexpr::error::Result<serde_json::Value> {}

#[test]
fn it_works() {
    let mut arg = serde_json::json!({
        "a":{
            "b":8
        }
    });
    let v = gen(&mut arg);
    println!("{}", v.unwrap());
}
