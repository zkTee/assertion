use std::collections::HashMap;
use std::{
    fmt::Debug,
    string::{String, ToString},
    vec::Vec,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Op {
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = ">=")]
    GreaterEq,
    #[serde(rename = "<=")]
    LessEq,
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEq,
    #[serde(rename = "contains")]
    Contains,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogicOp {
    Or,
    And,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LogicItem {
    Single {
        src: String,
        op: Op,
        dsc: String,
    },
    Multi {
        logic_op: LogicOp,

        #[serde(default)]
        items: Vec<Box<LogicItem>>,
    },
    MultiMap(HashMap<LogicOp, Vec<Box<LogicItem>>>),
}

impl LogicItem {
    pub fn new_multi(logic_op: LogicOp) -> Self {
        Self::Multi {
            logic_op,
            items: vec![],
        }
    }

    pub fn new_single<T: ToString>(src: T, op: Op, dsc: T) -> Self {
        Self::Single {
            src: src.to_string(),
            op,
            dsc: dsc.to_string(),
        }
    }
    pub fn add_item(mut self, item: LogicItem) -> Self {
        match &mut self {
            Self::Multi { items, .. } => items.push(Box::new(item)),
            Self::MultiMap(_map) => {
                todo!("insert to map")
            }
            Self::Single { .. } => unreachable!(),
        }
        self
    }
}

pub trait Logic {
    fn eval(&self) -> bool;
}

impl Logic for LogicItem {
    fn eval(&self) -> bool {
        match self {
            Self::Single { src, op, dsc } => match op {
                Op::Equal => src == dsc,
                Op::GreaterEq => src >= dsc,
                Op::LessEq => src <= dsc,
                Op::GreaterThan => src > dsc,
                Op::LessThan => src < dsc,
                Op::NotEq => src != dsc,
                Op::Contains => {
                    let arr: Vec<String> = serde_json::from_str(dsc).unwrap();
                    arr.contains(src)
                },
            },
            Self::Multi { logic_op, items } => match logic_op {
                LogicOp::Or => items.iter().any(|item| item.eval()),
                LogicOp::And => !items.iter().all(|item| !item.eval()),
            },
            Self::MultiMap(map) => {
                if map.is_empty() {
                    return false;
                }
                // TODO: make sure the map length is 1
                let (logic_op, items) = map.iter().next().unwrap();
                match logic_op {
                    LogicOp::Or => items.iter().any(|item| item.eval()),
                    LogicOp::And => !items.iter().any(|item| !item.eval()),
                }
            }
        }
    }
}
