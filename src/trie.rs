// use crate::lex::Token;
// use std::iter::Iterator;

// #[derive(Debug, Clone)]
// pub struct Trie {
//     len: usize,
//     node: TrieNode,
// }

// #[derive(Debug, Clone)]
// struct TrieNode {
//     key: char,
//     val: Token,
//     children: [Option<Box<TrieNode>>; 16],
// }

// impl Trie {
//     // pub const fn new() -> Self {
//     //     Self {
//     //         len: 0,
//     //         node: TrieNode {
//     //             key: '\0',
//     //             val: Token::Invalid,
//     //             children: [None; 16],
//     //         },
//     //     }
//     // }

//     // pub const fn from(keywords: &'a [&'a str], tokens: &'a [&'a Token]) -> Self {
//     //     Self {
//     //         len: 0,
//     //         node: TrieNode {
//     //             key: '\0',
//     //             val: Token::Invalid,
//     //             children: None,
//     //         },
//     //     }
//     // }

//     pub fn add(&mut self, key: &str, tok: Token) {
//         self.node.add(key, tok)
//     }
// }

// impl TrieNode {
//     fn add(&mut self, key: &str, tok: Token) {
//         // let maybe_next = key.chars().next().map(|c| (c, &key[c.len_utf8()..]));
//         let (ch, next) = match key.chars().next().map(|c| (c, &key[c.len_utf8()..])) {
//             Some((ch, s)) => {
//                 println!("first: {}, rest: {}", ch, s);
//                 (ch, s)
//             }
//             None => panic!("end of key reached"),
//         };
//         // if ch != self.key {
//         //     panic!("key was put in the wrong bucket");
//         // }
//         if next.len() == 0 {
//             panic!("reached end of key");
//             // return;
//         }
//         let mut i = 0;
//         let len = self.children.len();
//         while i < len {
//             i += 1;
//         }
//         // while let Some(child) = self.children.next() {}
//     }
// }

// #[cfg(test)]
// mod tests {
//     // use super::Trie;
//     // use crate::lex::Token;

//     // #[test]
//     // fn test_trie() {
//     //     let mut trie = Trie::new();
//     //     println!("{}", trie.len);
//     //     trie.add("mod", Token::Modulus);
//     // }
// }
