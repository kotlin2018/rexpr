use rexpr::runtime::RExprRuntime;
use serde_json::json;

fn main() {
    let runtime = RExprRuntime::new();
    let result = runtime.eval("1+1", &serde_json::json!(null)).unwrap();
    println!("result: {}", result);
}