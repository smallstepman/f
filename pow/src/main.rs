use std::{ops::RangeInclusive, str::FromStr};

use sha3::{Digest, Sha3_256};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "simple_pow")]
struct Opt {
    input: HexData,
}

struct HexData(Vec<u8>);

impl FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s).map(HexData)
    }
}

impl HexData {
    fn b64decode(self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(self.0)
    }
}

struct Prefix(RangeInclusive<u32>);

impl Prefix {
    pub fn new() -> Self {
        Prefix(0..=u32::max_value())
    }
}

impl Iterator for Prefix {
    type Item = [u8; 4];

    fn next(&mut self) -> Option<[u8; 4]> {
        Some(self.0.next().unwrap().to_be_bytes())
    }
}

fn last_two(arr: &[u8]) -> [u8; 2] {
    [arr[arr.len() - 2], arr[arr.len() - 1]]
}

fn main() {
    let input = match Opt::from_args().input.b64decode() {
        Ok(v) => v,
        Err(e) => {
            println!("Input string in not base64 encoded, {}", e);
            std::process::exit(0); // 256 for windows
        }
    };
    let prefix = Prefix::new();
    let target: [u8; 2] = [
        i64::from_str_radix("ca", 16).unwrap() as u8,
        i64::from_str_radix("fe", 16).unwrap() as u8,
    ];
    for p in prefix {
        let mut hasher = Sha3_256::new();
        let prefixed_input = [&p[..], &input[..]].concat();
        hasher.update(prefixed_input);
        let result = hasher.finalize();
        if last_two(&result) == target {
            println!("{}", hex::encode(result));
            println!("{:08x}", u32::from_be_bytes(p));
        }
    }
}

// test
