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
        dst: String,
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

    pub fn new_single<T: ToString>(src: T, op: Op, dst: T) -> Self {
        Self::Single {
            src: src.to_string(),
            op,
            dst: dst.to_string(),
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
            Self::Single { src, op, dst } => match op {
                Op::Equal => src == dst,
                Op::GreaterEq => src >= dst,
                Op::LessEq => src <= dst,
                Op::GreaterThan => src > dst,
                Op::LessThan => src < dst,
                Op::NotEq => todo!(),
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
