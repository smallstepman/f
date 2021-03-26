use sha3::{Digest, Sha3_256};
use std::{fmt::Write, num::ParseIntError, str::FromStr};
use structopt::StructOpt;
// use std::i64;
// use std::char::from_u32;
// use base64::decode;

#[derive(Debug, StructOpt)]
#[structopt(name = "hire_me")]
struct Opt {
    input: HexData,
}

#[derive(Debug)]
struct HexData(Vec<u8>);

impl FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s).map(HexData)
        //     (0..s.len())
        //         .step_by(2)
        //         .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        //     // .collect()
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt.input);
}
type U32Iter = impl Iterator<Item = u32>;

pub struct Prefix(U32Iter);

impl Iterator for Prefix {
    type Item = [u8; 4];

    fn next(&mut self) -> u8 {
        self.0.next().map(|n| n.to_be_bytes())
    }
}

impl Prefix {
    pub fn new() -> Self {
        0..=u32::max_value()
    }
    pub fn next_u8x4(&mut self) -> Option<[u8; 4]> {
        self.0.next().map(|n| n.to_be_bytes())
    }
}

// struct Prefix {}
// impl Iterator for Prefix {
//     type = [u8;4];
//     fn next() {

//     }
// }

// fn increase(v: &mut Vec<u8>) -> &mut Vec<u8> {
//     if v[v.len() - 1] == 255 {
//         // carry over
//         //panic!("arst");
//         v
//     } else {
//         v[3] += 1;
//         v
//     }
// }

// fn main() {
//     let input = "59584a7a64413d3d";
//     let input = decode_hex(input).unwrap();
//     let mut prefix: Vec<u8> = vec![0,0,0,0];
//     let target: [u8; 2] = [i64::from_str_radix("ca", 16).unwrap() as u8,
//                            i64::from_str_radix("fe", 16).unwrap() as u8];
//     println!("target trailing bytes: {:?}", target);
//     // create a SHA3-256 object
//     println!("{:?}", input);
//     // write input message
//     loop {
//         let mut hasher = Sha3_256::new();
//         increase(&mut prefix);
//         let result = [prefix, input].concat();
//         println!("{:?}", result);
//         hasher.update(result);
//         let result = hasher.finalize();
//         println!("{:?}", result);
//     }

//     // hasher.update(input);
//     // read hash digest
//     // let v = hex_literal::hex!("a34b2332")[..];
//     // assert_eq!(result[..], hex!("
//     //     3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532
//     // ")[..]);

//     println!("Hello, world!");
// }

// // input = bash64(str).hex()
// // sha256(prefix + input) = result
// // result[-4:] == cafe

// //     output: result , prefix
