use std::collections::HashSet;

fn get_priority(c: &char) -> i32 {
    (c.to_owned() as i32) - if c.is_uppercase() { 38 } else { 96 }
}

#[aoc(day3, part1, Chars)]
pub fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, l| {
        let (first, second) = l.split_at(l.len() / 2);
        let first_compartment: HashSet<char> = first.chars().into_iter().collect();
        let second_compartment: HashSet<char> = second.chars().into_iter().collect();
        let sum = acc
            + first_compartment
                .intersection(&second_compartment)
                .map(|c| get_priority(c))
                .sum::<i32>();
        sum
    })
}

use itertools::Itertools;
#[aoc(day3, part2, Chars)]
pub fn part2(input: &str) -> i32 {
    input.lines().into_iter().chunks(3).into_iter().map(|c| {
        let mut sets = c.map(|e| e.chars().into_iter().collect::<HashSet<char>>());
        let first_set = sets.next().unwrap();
        sets.into_iter().fold(first_set, |acc, set| {
            acc.intersection(&set.to_owned()).into_iter().cloned().collect()
        }).into_iter().map(|x| get_priority(&x)).sum::<i32>()
    }).sum()
}
