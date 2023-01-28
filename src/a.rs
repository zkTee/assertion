use crate::LogicItem;
use crate::assertion::*;

pub fn a() {
    let data = r#"
{
"or": [
  {
    "src": "a",
    "op": ">=",
    "dst": "2"
  },
  {
    "and": [
      {
        "src": "c",
        "op": "<",
        "dst": "10"
      }
    ]
  },
  {
    "src": "b",
    "op": ">",
    "dst": "3"
  }
]
}
"#;
    let logic_item: LogicItem = serde_json::from_str(&data).unwrap();
    println!("logic_item: {:?}", logic_item);
	println!("eval : {}", logic_item.eval());

    let logic_1 = LogicItem::new_single("a", Op::GreaterEq, "2");
    let logic_2 = LogicItem::new_single("c", Op::LessThan, "10");
    let logic_3 = LogicItem::new_single("b", Op::GreaterThan, "3");

    let m_logic_1 = LogicItem::new_multi(LogicOp::And).add_item(logic_2.clone());
    let m_logic = LogicItem::new_multi(LogicOp::Or)
        .add_item(logic_1.clone())
        .add_item(m_logic_1)
        .add_item(logic_3.clone());
    let s = serde_json::to_string_pretty(&m_logic).unwrap();
    println!("s: {}", s);
}