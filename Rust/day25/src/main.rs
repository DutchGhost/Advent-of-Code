extern crate libaoc;

use libaoc::convert::TryConvert;

const PUZZLE: &'static str = include_str!("Input.txt");

mod state;
use state::Block;
use state::*;

fn main() {
    let mut arr = [
        Block::new(),
        Block::new(),
        Block::new(),
        Block::new(),
        Block::new(),
        Block::new(),
    ];

    PUZZLE.split("\n,").try_convert_into_slice(&mut arr);

    let mut cpu = CPU::new(arr);
    for _ in 0..12794428 {
        cpu.run();
    }
    println!("{:?}", cpu.count_one());
}
