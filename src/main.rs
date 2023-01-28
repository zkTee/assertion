mod assertion;
use assertion::*;

fn _1() {
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

fn _2() {
    let data = r#"
{
"and": [
  {
         "or": [
            {
               "src": "$network",
               "op": "==",
               "dsc": "ethereum"
            },
            {
               "src": "$network",
               "op": "==",
               "dsc":"litentry"
            },
            {
               "src": "$network",
               "op": "==",
               "dsc": "litmus"
            }
         ]
      },
      {
         "src": "$txAmount",
         "op": ">=",
         "dsc": "500"
      },
      {
         "src":"$txAmount",
         "op":"<",
         "dsc":"1000"
      }
   ]
}"#;
    let logic_item: Result<LogicItem, _> = serde_json::from_str(data);
    println!("{:?}", logic_item);

    let logic_1 = LogicItem::new_single("$network", Op::Equal, "ethereum");
    let logic_2 = LogicItem::new_single("$network", Op::Equal, "litentry");
    let logic_3 = LogicItem::new_single("$network", Op::Equal, "litmus");
    let logic_4 = LogicItem::new_single("$txAmount", Op::GreaterThan, "500");
    let logic_5 = LogicItem::new_single("$txAmount", Op::LessThan, "1000");

    let m_logic_1 = LogicItem::new_multi(LogicOp::Or)
        .add_item(logic_1)
        .add_item(logic_2)
        .add_item(logic_3);
    let m_logic = LogicItem::new_multi(LogicOp::And)
        .add_item(m_logic_1)
        .add_item(logic_4)
        .add_item(logic_5);
    let s = serde_json::to_string_pretty(&m_logic).unwrap();
    println!("{}", s);
}

fn main() {
    _2();
}
