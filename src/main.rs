mod assertion;
use assertion::*;

fn main() {
    let data = r#"
{
  "or": [
    {
      "src": "a",
      "op": ">=",
      "dst": "2"
    },
    {
      "add": [
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


    let logic_1 = LogicItem::new_single("a", Op::BiggerEqual, "2");
    let logic_2 = LogicItem::new_single("c", Op::LowerThan, "10");
    let logic_3 = LogicItem::new_single("b", Op::BiggerThan, "3");

    let m_logic_1 = LogicItem::new_multi(LogicOp::Add).add_item(logic_2.clone());
    let m_logic = LogicItem::new_multi(LogicOp::Or)
        .add_item(logic_1.clone())
        .add_item(m_logic_1)
        .add_item(logic_3.clone());
    let s = serde_json::to_string_pretty(&m_logic).unwrap();
    println!("s: {}", s);

    // // use map format
    // println!("==========================");
    // let mut h1 = HashMap::new();
    // h1.insert(LogicOp::Add, vec![Box::new(logic_2)]);
    // let m_logic_1 = LogicItem::MultiMap(h1);
    // let mut h2 = HashMap::new();
    // h2.insert(
    //     LogicOp::Or,
    //     vec![
    //         Box::new(logic_1.clone()),
    //         Box::new(m_logic_1),
    //         Box::new(logic_3.clone()),
    //     ],
    // );
    // let m_logic = LogicItem::MultiMap(h2);
    // let s = serde_json::to_string_pretty(&m_logic).unwrap();
    // println!("{}", s);
}