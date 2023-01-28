use crate::Assertion;
use crate::assertion::*;

pub fn b() {
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
    let logic_item: Result<Assertion, _> = serde_json::from_str(data);
    println!("{:?}", logic_item);
    println!("eval : {}", logic_item.unwrap().eval());

    let logic_1 = Assertion::new_item("$network", Op::Equal, "ethereum");
    let logic_2 = Assertion::new_item("$network", Op::Equal, "litentry");
    let logic_3 = Assertion::new_item("$network", Op::Equal, "litmus");
    let logic_4 = Assertion::new_item("$txAmount", Op::GreaterThan, "500");
    let logic_5 = Assertion::new_item("$txAmount", Op::LessThan, "1000");

    let m_logic_1 = Assertion::new_item_array(LogicOp::Or)
        .add_item(logic_1)
        .add_item(logic_2)
        .add_item(logic_3);
    let m_logic = Assertion::new_item_array(LogicOp::And)
        .add_item(m_logic_1)
        .add_item(logic_4)
        .add_item(logic_5);
    let s = serde_json::to_string_pretty(&m_logic).unwrap();
    println!("s = {}", s);
}
