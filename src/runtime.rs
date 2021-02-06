use crate::ast::Node;
use crate::cache::RWLockMapCache;
use crate::lexer::lexer;
use crate::parser::parse;
use crate::token::TokenMap;
use dashmap::DashMap;
use serde_json::Value;

/// the express engine for  exe code on runtime
#[derive(Debug)]
pub struct RExprRuntime {
    pub expr_cache: DashMap<String, Node>,
    pub token_map: TokenMap<'static>,
}

impl RExprRuntime {
    pub fn new() -> Self {
        return Self {
            expr_cache: DashMap::new(),
            token_map: TokenMap::new(),
        };
    }

    ///eval express with arg value,if cache have value it will no run lexer expr.
    pub fn eval(&self, expr: &str, arg: &Value) -> Result<Value, crate::error::Error> {
        match self.expr_cache.get(expr) {
            Some(v) => {
                return v.eval(arg);
            }
            _ => {
                let node = self.parse(expr)?;
                self.expr_cache.insert(expr.to_string(), node.clone());
                return node.eval(arg);
            }
        }
    }

    /// no cache mode to run engine
    pub fn eval_no_cache(
        &self,
        lexer_arg: &str,
        arg: &Value,
    ) -> Result<Value, crate::error::Error> {
        let tokens = lexer(lexer_arg, &self.token_map)?;
        let node = parse(&self.token_map, &tokens, lexer_arg)?;
        return node.eval(arg);
    }

    /// parse get node
    pub fn parse(&self, lexer_arg: &str) -> Result<Node, crate::error::Error> {
        let tokens = lexer(lexer_arg, &self.token_map)?;
        let node = parse(&self.token_map, &tokens, lexer_arg)?;
        return Ok(node);
    }
}

#[cfg(test)]
mod test {
    use crate::bencher::QPS;
    use crate::runtime::RExprRuntime;
    use std::sync::Arc;
    use std::thread::{sleep, spawn};
    use std::time::Duration;
    use crate::eval::eval;
    use crate::lexer::lexer;

    //cargo test --release --package rexpr --lib runtime::test::test_bench_func --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    fn test_bench_func() {
        let runtime = RExprRuntime::new();
        let func=runtime.parse("1+1").unwrap();

        let a=serde_json::json!(1);
        let b=serde_json::json!(1);
        let total = 1000000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            //use Time: 7.2385ms ,each:7 ns/op
            //serde_json::json!(a.as_i64().unwrap()+b.as_i64().unwrap());
            //(Windows10 6Core16GBMem) use Time: 41.8378ms ,each:41 ns/op
            func.eval( &serde_json::Value::Null).unwrap();

            //eval(&a,&b,"+");
        }
        now.time(total);
        now.qps(total);
    }

    //cargo test --release --package rexpr --lib runtime::test::test_bench --no-fail-fast -- --exact -Z unstable-options --show-output
    #[test]
    fn test_bench() {
        let runtime = RExprRuntime::new();
        runtime.eval("1+1", &serde_json::Value::Null);
        runtime.eval("1+1", &serde_json::Value::Null);

        let total = 1000000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            //(Windows10 6Core16GBMem) use Time: 84.0079ms ,each:84 ns/op use QPS: 11900823 QPS/s
            let r = runtime.eval("1+1", &serde_json::Value::Null).unwrap(); //use Time: 1.5752844s ,each:1575 ns/op use QPS: 634793 QPS/s
                                                                            //println!("{}",r);
        }
        now.time(total);
        now.qps(total);
    }

    #[test]
    fn test_thread_race() {
        let runtime = Arc::new(RExprRuntime::new());
        let r1 = runtime.clone();
        spawn(move || {
            let total = 1000000;
            for _ in 0..total {
                let r = r1.eval("1+1", &serde_json::Value::Null).unwrap();
            }
        });
        let r2 = runtime.clone();
        spawn(move || {
            let total = 1000000;
            for _ in 0..total {
                let r = r2.eval("1+1", &serde_json::Value::Null).unwrap();
            }
        });
        sleep(Duration::from_secs(10));
    }
}
