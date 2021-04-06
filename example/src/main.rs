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

// #[py_sql(   "select * from biz_activity where delete_flag = 0
//                   if @.name.str() != '':
//                     and name=#{@.name}")]

#[expr("@['a']['b'].i64() != &null")]
pub fn gen(arg: &serde_json::Value) -> rexpr::error::Result<serde_json::Value> {}

#[test]
fn it_works() {
    let arg = serde_json::json!({
        "a":{
            "b":8
        }
    });
    let v = gen(&arg);
    println!("{}", v.unwrap());
}
