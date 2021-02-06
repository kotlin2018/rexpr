#[cfg(test)]
mod test {
    use crate::bencher::QPS;
    use crate::lexer;
    use crate::token::TokenMap;
    use serde_json::json;

    #[test]
    fn test_lexer() {
        let node = lexer::lexer_parse_node("-1 == -a", &TokenMap::new()).unwrap();
        println!("{:#?}", &node);
        let john = json!({
            "a":1,
            "name": "John Doe",
            "age": {
               "yes":"sadf"
            },
             "sex":{
                "a":"i'm a",
                "b":"i'm b",
             },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });
        println!("result >>>>>>>>>>   =  {}", node.eval(&john).unwrap());
    }

    #[test]
    fn test_benchmark_parse() {
        let total = 10000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            let box_node = lexer::lexer("1+1", &TokenMap::new()).unwrap();
        }
        now.time(total)
    }

    #[test]
    fn test_benchmark() {
        let box_node = lexer::lexer_parse_node("1+1", &TokenMap::new()).unwrap();
        let john = json!({
            "name": "John Doe",
        });
        let total = 10000000;
        let now = std::time::Instant::now();
        for _ in 0..total {
            box_node.eval(&john);
        }
        now.time(total)
    }
}
