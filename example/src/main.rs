#[macro_use]
extern crate rexpr;

use rexpr::runtime::RExprRuntime;

fn main() {
    let runtime = RExprRuntime::new();
    let result = runtime.eval("1+1", &serde_json::json!(null)).unwrap();
    println!("result: {}", result);
}

// #[py_sql(   "select * from biz_activity where delete_flag = 0
//                   if name.str() != '':
//                     and name=#{name}")]

#[expr("!a.c.bool()")]
pub fn gen(arg: &serde_json::Value) -> rexpr::error::Result<serde_json::Value> {}

#[test]
fn it_works() {
    let arg = serde_json::json!({
        "a":{
            "arr":[1,2,3],
            "b":"8"
        }
    });
    let v = gen(&arg);
    println!("{}", v.unwrap());
}

#[test]
fn bench() {
    let arg = serde_json::json!({
        "a":{
            "arr":[1,2,3],
            "b":"8",
            "c":true
        }
    });
    gen(&arg);
    bench!(100000,{
       gen(&arg);
    });
}

