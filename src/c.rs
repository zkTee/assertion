use crate::LogicItem;
use crate::assertion::*;

pub fn c() {
//     let data = r#"
// {
// "or": [
//         {
//             "src": "$networka",
//             "op": ">=",
//             "dsc": "2"
//         },
//         {
//             "src": "a",
//             "op": ">=",
//             "dsc": "2"
//         },
//         {
//             "and": [
//                 {
//                     "src": "a",
//                     "op": ">=",
//                     "dsc": "2"
//                 } 
//             ]
//         }
//     ]
// }
// "#;
let data = r#"
{
    "and": [

             {
                "src": "$txAmount",
                "op": ">=",
                "dsc": "500"
             },
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
                "src":"$txAmount",
                "op":"<",
                "dsc":"1000"
             }
        ]
}
"#;

let data = r#"
{
    "and": [

             {
                "src": "$txAmount",
                "op": ">=",
                "dsc": "500"
             }
        ]
}
"#;
    let data = r#"
{
"or": [
        {
            "src": "$networka",
            "op": ">=",
            "dsc": "2"
        }
    ]
}"#;
    let logic_item: LogicItem = serde_json::from_str(&data).unwrap();
    println!("logic_item: {:?}", logic_item);
	println!("eval : {}", logic_item.eval());
}