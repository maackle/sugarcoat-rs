// struct Item<H = Hash> {
//     pub seq: u32,
//     pub hash: H,
//     pub prev: Option<H>,
//     pub data: Vec<u8>,
//     pub author: H,
// }

// struct ChainItem(Item<SimpleHash>);

// impl ChainItem {
//     pub fn new(x: u8) -> Self {
//         Self(Item {
//             seq: x.into(),
//             hash: SimpleHash::new(x),
//             prev: if x == 0 {
//                 None
//             } else {
//                 Some(SimpleHash::new(x - 1))
//             },
//             data: vec![],
//             author: SimpleHash::new(0),
//         })
//     }
// }

// fn handle_b(b: ChainItem) {}
