use std::ops::RangeInclusive;

type ElfPair = (RangeInclusive<i32>, RangeInclusive<i32>);

fn get_range(s: &str) -> RangeInclusive<i32> {
    let (first, second) = s.split_once('-').unwrap();
    first.parse::<i32>().unwrap() ..= second.parse::<i32>().unwrap()
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<ElfPair> {
    input.lines().into_iter().map(|l| {
        let (first, second) = l.split_once(',').unwrap();
        (get_range(first), get_range(second))
    }).collect()
}

//#[aoc(day4, part1)]
//pub fn part1(input: &[ElfPair]) -> i32 {
//    input.into_iter().map(|pair| if )
//}
