const PUZZLE: &'static str = include_str!("Input.txt");

extern crate libaoc;
mod particle;

mod prelude {
    pub use libaoc::convert::TryConvert;
    pub use particle::GPU;
    pub use std::str::FromStr;
}

use prelude::*;

fn main() {
    let mut gpu0 = GPU::from_str(PUZZLE).unwrap();
    let mut gpu1 = GPU::from_str(PUZZLE).unwrap();
    for _ in 0..1000 {
        gpu0.update();
        gpu1.collisionupdate();
    }
    println!("part 1: {}", gpu0.closest());
    println!("part 2: {}", gpu1.countparticles());
}
