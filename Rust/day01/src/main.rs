#![feature(slice_patterns)]
#![feature(test)]
extern crate test;
const PUZZLE: &'static str = include_str!("Input.txt");
const BPUZZLE: &'static [u8; 2190] = include_bytes!("Input.txt");

fn main() {
    use std::time::Instant;

    let q = Instant::now();
    println!("{}", optimized_andpercent_unrolled(BPUZZLE, BPUZZLE.len() >> 1));
    println!("{:?}", q.elapsed());


    println!("{}", summmenize_andpercent(BPUZZLE, 1));
    let s = Instant::now();
    println!("{}", optimized_andpercent(BPUZZLE, PUZZLE.len() >> 1));
    println!("{:?}", s.elapsed());


    println!("day 1.1: {}", bytes_summenize(BPUZZLE, 1));
    let t = Instant::now();
    println!("day 1.2: {}", bytes_optimized(BPUZZLE, BPUZZLE.len() >> 1));
    println!("{:?}", t.elapsed());

}

#[inline]
fn summmenize_andpercent(input: &[u8; 2190], skip: usize) -> u32 {
    let mut totall: u32 = 0;

    for (c1, c2) in input.iter().zip(input.iter().skip(skip)) {
        totall += ((*c1 as i8 - 48) & -((c1 == c2) as i8)) as u32
    }
    //assert!(input.len() > 2189);
    totall += ((input[0] - 48) as i32 & -((input[0] == input[2189]) as i32)) as u32;
    totall
}

#[inline]
fn optimized_andpercent(input: &[u8; 2190], half: usize) -> u32 {
    let (head, tail) = input.split_at(half);

    head
        .iter()
        .zip(tail.iter())
        .map(|(c1, c2)| ((*c1 as i8 - 48) & -((c1 == c2) as i8)) as u32)
        .sum::<u32>() << 1
   
}
/// take an &str, loop over the chars,
/// and zip with an infinite version of itself that skips for `skip`.
#[inline]
fn summenize(input: &str, skip: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(skip))
        .filter_map(|(first, second)| if first == second { first.to_digit(10) } else { None })
        .sum()
}

/// We devide the str in half, check for each element.
/// at the end we bitshift by 1 to the left (multiply by 2),
/// every item we found in the first half, will also be the same fore the second half
#[inline]
fn optimized(input: &str, half: usize) -> u32 {
    let (head, tail) = input.split_at(half);
    head
        .chars()
        .zip(tail.chars())
        .filter_map(|(first, second)| if first == second { first.to_digit(10)} else { None })
        .sum::<u32>() << 1
}

fn bytes_summenize(input: &[u8; 2190], skip: usize) -> u32 {
    input
        .iter()
        .zip(input.iter().cycle().skip(skip))
        .filter_map(|(first, second)| if first == second { (*first as char).to_digit(10) } else { None })
        .sum::<u32>()
}

fn bytes_optimized(input: &[u8; 2190], half: usize) -> u32 {
    let (head, tail) = input.split_at(half);
    head
        .iter()
        .zip(tail.iter())
        .filter_map(|(first, second)| if first == second { (*first as char).to_digit(10)} else { None })
        .sum::<u32>() << 1
}

// #[inline]
// pub fn optimized_andpercent_unrolled(input: &[u8; 2190], HALF: usize) -> u32 {
//     let mut totall = 0;
//     let (head, tail) = input.split_at(HALF);
    
//     unsafe {
//         for (lhs, rhs) in head.chunks(5).zip(tail.chunks(5)) {
//             //assert!(lhs.len() == 5 && rhs.len() == 5);
//             totall +=  (((*lhs.get_unchecked(0) as i8 - 48) & -((*lhs.get_unchecked(0) == *rhs.get_unchecked(0)) as i8)) +
//                         ((*lhs.get_unchecked(1) as i8 - 48) & -((*lhs.get_unchecked(1) == *rhs.get_unchecked(1)) as i8)) +
//                         ((*lhs.get_unchecked(2) as i8 - 48) & -((*lhs.get_unchecked(2) == *rhs.get_unchecked(2)) as i8)) +
//                         ((*lhs.get_unchecked(3) as i8 - 48) & -((*lhs.get_unchecked(3) == *rhs.get_unchecked(3)) as i8)) +
//                         ((*lhs.get_unchecked(4) as i8 - 48) & -((*lhs.get_unchecked(4) == *rhs.get_unchecked(4)) as i8))) as u32;
//         }
//     }
//     totall << 1
// }
const SUB: i8 = 48;
pub fn optimized_andpercent_unrolled(input: &[u8; 2190], HALF: usize) -> u32 {
    let mut totall = 0;
    unsafe {
        let head = input.get_unchecked(0..1095);
        let tail = input.get_unchecked(1095..);

        head.chunks(15).zip(tail.chunks(15)).for_each(|(lhs, rhs)|  {
            totall +=
                (
                        ((*lhs.get_unchecked(0) as i8 - SUB) & -((*lhs.get_unchecked(0) == *rhs.get_unchecked(0)) as i8)) +
                        ((*lhs.get_unchecked(1) as i8 - SUB) & -((*lhs.get_unchecked(1) == *rhs.get_unchecked(1)) as i8)) +
                        ((*lhs.get_unchecked(2) as i8 - SUB) & -((*lhs.get_unchecked(2) == *rhs.get_unchecked(2)) as i8)) +
                        ((*lhs.get_unchecked(3) as i8 - SUB) & -((*lhs.get_unchecked(3) == *rhs.get_unchecked(3)) as i8)) +
                        ((*lhs.get_unchecked(4) as i8 - SUB) & -((*lhs.get_unchecked(4) == *rhs.get_unchecked(4)) as i8)) +
                        ((*lhs.get_unchecked(5) as i8 - SUB) & -((*lhs.get_unchecked(5) == *rhs.get_unchecked(5)) as i8)) +
                        ((*lhs.get_unchecked(6) as i8 - SUB) & -((*lhs.get_unchecked(6) == *rhs.get_unchecked(6)) as i8)) +
                        ((*lhs.get_unchecked(7) as i8 - SUB) & -((*lhs.get_unchecked(7) == *rhs.get_unchecked(7)) as i8)) +
                        ((*lhs.get_unchecked(8) as i8 - SUB) & -((*lhs.get_unchecked(8) == *rhs.get_unchecked(8)) as i8)) +
                        ((*lhs.get_unchecked(9) as i8 - SUB) & -((*lhs.get_unchecked(9) == *rhs.get_unchecked(9)) as i8)) +
                        ((*lhs.get_unchecked(10) as i8 - SUB) & -((*lhs.get_unchecked(10) == *rhs.get_unchecked(10)) as i8)) +
                        ((*lhs.get_unchecked(11) as i8 - SUB) & -((*lhs.get_unchecked(11) == *rhs.get_unchecked(11)) as i8)) +
                        ((*lhs.get_unchecked(12) as i8 - SUB) & -((*lhs.get_unchecked(12) == *rhs.get_unchecked(12)) as i8)) +
                        ((*lhs.get_unchecked(13) as i8 - SUB) & -((*lhs.get_unchecked(13) == *rhs.get_unchecked(13)) as i8)) +
                        ((*lhs.get_unchecked(14) as i8 - SUB) & -((*lhs.get_unchecked(14) == *rhs.get_unchecked(14)) as i8))
                ) as u32;
        });
    }
    totall << 1
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;

    #[bench]
    fn bytes_summenize_part1(b: &mut Bencher) {
        b.iter(|| bytes_summenize(BPUZZLE, 1));
    }

    #[bench]
    fn bytes_summenize_part2(b: &mut Bencher) {
        b.iter(|| bytes_summenize(BPUZZLE, BPUZZLE.len() >> 1));
    }

    #[bench]
    fn bytes_fast_part2(b: &mut Bencher) {
        b.iter(|| bytes_optimized(BPUZZLE, BPUZZLE.len() >> 1));
    }

    #[bench]
    fn str_summenize_part1(b: &mut Bencher) {
        b.iter(|| summenize(PUZZLE, 1));
    }

    #[bench]
    fn str_summenize_part2(b: &mut Bencher) {
        b.iter(|| summenize(PUZZLE, PUZZLE.len() >> 1));
    }

    #[bench]
    fn str_fast_part2(b: &mut Bencher) {
        b.iter(|| optimized(PUZZLE, PUZZLE.len() >> 1));
    }

    #[bench]
    fn bytes_summenize_part1_andpercent(b: &mut Bencher) {
        b.iter(|| summmenize_andpercent(BPUZZLE, 1));
    }

    #[bench]
    fn bytes_optimized_part2_andpercent(b: &mut Bencher) {
        b.iter(|| optimized_andpercent(BPUZZLE, PUZZLE.len() >> 1));
    }

    #[bench]
    fn unrolled_optimized_part2_andpercent(b: &mut Bencher) {
        b.iter(|| optimized_andpercent_unrolled(BPUZZLE, BPUZZLE.len() >> 1));
    }
}