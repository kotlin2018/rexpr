use crate::access::AccessField;
use crate::ast::NodeType::{NArg, NBinary, NBool, NNull, NNumber, NOpt, NString};
use crate::error::Error;
use crate::eval::eval;
use crate::token::TokenMap;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::json;
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum NodeType {
    NArg = 1,
    NString = 2,
    NNumber = 3,
    NBool = 4,
    NNull = 5,
    NBinary = 6,
    NOpt = 7,
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            NArg => f.write_str("NArg"),
            NString => f.write_str("NString"),
            NNumber => f.write_str("NNumber"),
            NBool => f.write_str("NBool"),
            NNull => f.write_str("NNull"),
            NBinary => f.write_str("NBinary"),
            NOpt => f.write_str("NOpt"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub left: Option<Box<Node>>,
    pub value: Value,
    pub right: Option<Box<Node>>,
    pub node_type: NodeType,
}

impl Node {
    pub fn to_number(&self) -> f64 {
        return self.value.as_f64().unwrap_or(0.0);
    }
    pub fn to_string(&self) -> &str {
        return self.value.as_str().unwrap_or("");
    }
    pub fn to_arg(&self) -> &str {
        return self.value.as_str().unwrap_or("");
    }
    pub fn to_bool(&self) -> bool {
        return self.value.as_bool().unwrap_or(false);
    }
    pub fn to_token(&self) -> &str {
        return self.value.as_str().unwrap_or("");
    }
    pub fn node_type(&self) -> NodeType {
        return self.node_type.clone();
    }

    #[inline]
    pub fn equal_node_type(&self, arg: &NodeType) -> bool {
        return self.node_type == *arg;
    }

    pub fn is_value_node(&self) -> Option<Value> {
        if self.equal_node_type(&NBinary) {
            return Option::None;
        } else if self.equal_node_type(&NArg) {
            return Option::None;
        } else {
            return Option::Some(self.value.clone());
        }
    }

    #[inline]
    pub fn eval(&self, env: &Value) -> Result<Value, crate::error::Error> {
        if self.equal_node_type(&NBinary) {
            let left_v = self.left.as_ref().unwrap().eval(env)?;
            let right_v = self.right.as_ref().unwrap().eval(env)?;
            let token = self.to_string();
            return eval(&left_v, &right_v, token);
        } else if self.equal_node_type(&NArg) {
            return self.value.access_field(env);
        }
        return Result::Ok(self.value.clone());
    }

    pub fn token(&self) -> Option<&str> {
        return self.value.as_str();
    }

    pub fn new_null() -> Self {
        Self {
            value: Value::Null,
            left: None,
            right: None,
            node_type: NNull,
        }
    }
    pub fn new_arg(arg: &str) -> Self {
        let new_arg = arg.replace("]", "").replace("[", ".");
        let d: Vec<&str> = new_arg.split(".").collect();
        Self {
            value: json!(d),
            left: None,
            right: None,
            node_type: NArg,
        }
    }
    pub fn new_string(arg: &str) -> Self {
        Self {
            value: Value::String(arg.to_string()),
            left: None,
            right: None,
            node_type: NString,
        }
    }
    pub fn new_f64(arg: f64) -> Self {
        Self {
            value: json!(arg),
            left: None,
            right: None,
            node_type: NNumber,
        }
    }
    pub fn new_i64(arg: i64) -> Self {
        Self {
            value: json!(arg),
            left: None,
            right: None,
            node_type: NNumber,
        }
    }
    pub fn new_u64(arg: u64) -> Self {
        Self {
            value: json!(arg),
            left: None,
            right: None,
            node_type: NNumber,
        }
    }

    pub fn new_bool(arg: bool) -> Self {
        Self {
            value: Value::Bool(arg),
            left: None,
            right: None,
            node_type: NBool,
        }
    }
    pub fn new_binary(arg_lef: Node, arg_right: Node, token: &str) -> Self {
        Self {
            value: Value::from(token),
            left: Option::Some(Box::new(arg_lef)),
            right: Option::Some(Box::new(arg_right)),
            node_type: NBinary,
        }
    }
    pub fn new_token(arg: &str) -> Self {
        Self {
            value: Value::String(arg.to_string()),
            left: None,
            right: None,
            node_type: NOpt,
        }
    }

    pub fn parse(data: &str, token: &TokenMap) -> Result<Self, Error> {
        if data == "" || data == "null" {
            return Ok(Node::new_null());
        } else if let Ok(n) = data.parse::<bool>() {
            return Ok(Node::new_bool(n));
        } else if token.is_token(data) {
            return Ok(Node::new_token(data));
        } else if (data.starts_with("'") && data.ends_with("'"))
            || (data.starts_with("`") && data.ends_with("`"))
        {
            return Ok(Node::new_string(&data[1..data.len() - 1]));
        } else if let Ok(n) = data.parse::<f64>() {
            if data.find(".").unwrap_or(0) != 0 {
                return Ok(Node::new_f64(n));
            } else {
                return Ok(Node::new_i64(n as i64));
            }
        } else {
            if Self::is_arg(data) {
                return Ok(Node::new_arg(data));
            } else {
                return Err(Error::from(format!(
                    "[rexpr] arg token not allow! token: {}",
                    data
                )));
            }
        }
    }

    fn is_arg(arg: &str) -> bool {
        for c in arg.chars() {
            match c {
                //a~z
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
                //A~Z
                'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' |
                //0~9
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
                //_ .
                '_' | '.' | '[' | ']'
                => {
                    //nothing to do
                }
                _ => {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Node;
    use crate::ast::NodeType::NArg;
    use crate::token::TokenMap;

    #[test]
    fn test_parse() {
        let token_map = TokenMap::new();
        let node = Node::parse("'123'", &token_map).unwrap();
        assert_eq!(node.value.as_str().unwrap(), "123");

        let node = Node::parse("'12\'\'3'", &token_map).unwrap();
        assert_eq!(node.value.as_str().unwrap(), "12\'\'3");
    }

    #[test]
    fn test_parse_arg() {
        let token_map = TokenMap::new();
        let node = Node::parse("test", &token_map).unwrap();
        assert_eq!(node.node_type, NArg);
    }
}
