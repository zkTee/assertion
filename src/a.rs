use crate::assertion::*;
use crate::Assertion;

pub fn a() {
    let data = r#"
{
"or": [
  {
    "src": "a",
    "op": ">=",
    "dsc": "2"
  },
  {
    "and": [
      {
        "src": "c",
        "op": "<",
        "dsc": "10"
      }
    ]
  },
  {
    "src": "b",
    "op": ">",
    "dsc": "3"
  }
]
}
"#;
    let logic_item: Assertion = serde_json::from_str(&data).unwrap();
    println!("logic_item: {:?}", logic_item);
    println!("eval : {}", logic_item.eval());

    let logic_1 = Assertion::new_item("a", Op::GreaterEq, "2");
    let logic_2 = Assertion::new_item("c", Op::LessThan, "10");
    let logic_3 = Assertion::new_item("b", Op::GreaterThan, "3");

    let m_logic_1 = Assertion::new(LogicOp::And, vec![Box::new(logic_2.clone())]);

    let m_logic = Assertion::new(LogicOp::Or, vec![Box::new(logic_1.clone()), Box::new(m_logic_1), Box::new(logic_3.clone())]);
    let s = serde_json::to_string_pretty(&m_logic).unwrap();
    println!("s: {}", s);
}
