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

#[expr("1+2+&arg['a']['b'].as_i64().unwrap_or(0)")]
pub fn gen(arg: &mut serde_json::Value) -> rexpr::error::Result<serde_json::Value> {
    let a_b = &arg["a"]["b"];
    if a_b.is_string() {
        let mut owner1 = String::new();
        owner1 = owner1 + "1" + "2" + a_b.as_str().unwrap_or("");
    }
    Ok(serde_json::json!(1+2+a_b.as_i64().unwrap_or(0)))
}

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
