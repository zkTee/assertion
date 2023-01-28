use serde::{Deserialize, Serialize};
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LogicItem {
    Single {
        src: String,
        op: Op,
        dsc: String,
    },
    And {
        #[serde(rename = "and")]
        items: Vec<Box<LogicItem>>,
    },
    Or {
        #[serde(rename = "or")]
        items: Vec<Box<LogicItem>>,
    },
}

impl LogicItem {
    pub fn new_add() -> Self {
        Self::And { items: vec![] }
    }

    pub fn new_or() -> Self {
        Self::Or { items: vec![] }
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
            Self::Single { .. } => unreachable!(),
            Self::Or { items } => items.push(Box::new(item)),
            Self::And { items } => items.push(Box::new(item)),
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
                Op::GreaterThan => src > dsc,
                Op::LessThan => src < dsc,
                Op::GreaterEq => src >= dsc,
                Op::LessEq => src <= dsc,
                Op::Equal => src == dsc,
                Op::NotEq => src != dsc,
            },
            Self::And { items } => items.iter().all(|item| item.eval()),
            Self::Or { items } => items.iter().any(|item| item.eval()),
        }
    }
}
