extern crate libaoc;

use libaoc::convert::{Convert, TryConvert};

use std::iter::*;
use std::ops::Range;
use std::time::Instant;

const PUZZLE: &'static str = "31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33";
const INPUT_SIZE: usize = 16;

const BYTESPUZZLE: [u8; 49] = *b"31,2,85,1,80,109,35,63,98,255,0,13,105,254,128,33";
const SALT: [u8; 5] = [17, 31, 73, 47, 23];

const NUMS: [usize; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255,
];

fn parse_bytes(input: [u8; 49]) -> [usize; 49 + 5] {
    let mut arr = [0usize; 49 + 5];
    input
        .into_iter()
        .chain(SALT.iter())
        .cloned()
        .convert_into_slice(&mut arr);
    arr
}

//wrapper for the 'range' of nums that need to be changed.
enum Wrapper<T, I1: Iterator<Item = T>, I2: Iterator<Item = T>> {
    Wrapped(I1),
    Nonwrapped(I2),
}

impl<T, I1: Iterator<Item = T>, I2: Iterator<Item = T>> Iterator for Wrapper<T, I1, I2> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Wrapper::Nonwrapped(ref mut iter) => iter.next(),
            Wrapper::Wrapped(ref mut iter) => iter.next(),
        }
    }
}
//'Wrapped' goes from cpos to numslenght, and then from 0 to (len - (numslenght - cpos))
//'Nonwrapped' goes from cpos to cpos + len
#[inline]
fn wrapping(cpos: usize, len: usize, numslenght: usize) -> impl Iterator<Item = (usize, usize)> {
    if cpos + len < numslenght {
        Wrapper::Nonwrapped(
            (cpos..cpos + len)
                .zip((cpos..cpos + len).rev())
                .take(len / 2),
        )
    } else {
        let already_got = numslenght - cpos;

        Wrapper::Wrapped(
            (cpos..numslenght)
                .chain(0..(len - already_got))
                .zip((cpos..numslenght).chain(0..(len - already_got)).rev())
                .take(len / 2),
        )
    }
}

fn solve(rounds: i64, nums: &mut [usize], lenghts: &[usize]) -> usize {
    //assert!(nums.len() == 256);
    let mut cpos = 0;
    let mut skipsize = 0;
    let numslenght = nums.len();
    for _ in 0..rounds {
        for len in lenghts.iter() {
            wrapping(cpos, *len, numslenght).for_each(|(n1, n2)| nums.swap(n1, n2));
            cpos += *len + skipsize;
            cpos = cpos % numslenght;
            skipsize += 1;
        }
    }
    nums[0] * nums[1]
}

#[inline]
fn dense(nums: &[usize]) -> String {
    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |n, acc| n ^ acc))
        .map(|chunk| format!("{:02x}", chunk).to_lowercase())
        .collect()
}

fn main() {
    let mut nums_part1 = NUMS;
    let mut lenghts_part1: [usize; INPUT_SIZE] = [0; INPUT_SIZE];
    PUZZLE.split(",").try_convert_into_slice(&mut lenghts_part1);
    println!("part 1: {}", solve(1, &mut nums_part1, &lenghts_part1));

    let mut nums_part2 = NUMS;
    let lenghts_part2 = parse_bytes(BYTESPUZZLE);
    let rounds = 64;
    let started = Instant::now();
    solve(rounds, &mut nums_part2, &lenghts_part2);
    println!("part 2: {}", dense(&nums_part2));
    println!(
        "Calculated a hash of {} rounds in {:?} seconds",
        rounds,
        started.elapsed()
    );
}
