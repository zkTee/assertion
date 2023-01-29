use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fmt::Debug,
    string::{String, ToString},
    vec::Vec,
};

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
pub enum Assertion {
    Item {
        src: String,
        op: Op,
        dsc: String,
    },
    ItemArray {
        logic_op: LogicOp,

        #[serde(default)]
        items: Vec<Box<Assertion>>,
    },
    ItemMap(HashMap<LogicOp, Vec<Box<Assertion>>>),
}

impl Assertion {
    pub fn new_item_array(logic_op: LogicOp) -> Self {
        Self::ItemArray {
            logic_op,
            items: vec![],
        }
    }

    pub fn new_item<T: ToString>(src: T, op: Op, dsc: T) -> Self {
        Self::Item {
            src: src.to_string(),
            op,
            dsc: dsc.to_string(),
        }
    }
    pub fn add_item(mut self, item: Assertion) -> Self {
        match &mut self {
            Self::ItemArray { items, .. } => items.push(Box::new(item)),
            Self::ItemMap(_map) => {
                todo!("insert to map")
            }
            Self::Item { .. } => unreachable!(),
        }
        self
    }
}

pub trait Logic {
    fn eval(&self) -> bool;
}

impl Logic for Assertion {
    fn eval(&self) -> bool {
        println!("self: {:?}", self);

        match self {
            Self::Item { src, op, dsc } => match op {
                Op::Equal => src == dsc,
                Op::GreaterEq => src >= dsc,
                Op::LessEq => src <= dsc,
                Op::GreaterThan => src > dsc,
                Op::LessThan => src < dsc,
                Op::NotEq => src != dsc,
                Op::Contains => {
                    // let arr_src = serde_json::from_str(src).unwrap();
                    // let arr_dsc: Vec<dyn PartialEq> = serde_json::from_str(dsc).unwrap();
                    // let mut is_contains = true;
                    // for v in arr_src {
                    //     is_contains = arr_dsc.contains(&v) && is_contains;
                    // }

                    // is_contains

                    false
                }
            },
            Self::ItemArray { logic_op, items } => match logic_op {
                LogicOp::Or => items.iter().any(|item| item.eval()),
                LogicOp::And => !items.iter().all(|item| !item.eval()),
            },
            Self::ItemMap(map) => {
                if map.is_empty() {
                    return false;
                }
                // TODO: make sure the map length is 1
                assert!(map.len() >= 1);

                let (logic_op, items) = map.iter().next().unwrap();
                match logic_op {
                    LogicOp::Or => items.iter().any(|item| item.eval()),
                    LogicOp::And => !items.iter().any(|item| !item.eval()),
                }
            }
        }
    }
}
