#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut current_sum: i32 = 0;
    input.lines().for_each(|l| match l {
        "" => {
            if current_sum > sum {
                sum = current_sum
            }
            current_sum = 0;
        }
        _ => current_sum += l.parse::<i32>().unwrap(),
    });
    sum
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(0));
    heap.push(Reverse(0));
    heap.push(Reverse(0));
    let mut current_sum = 0;

    input.lines().for_each(|l| {
        println!("{current_sum}");
        match l {
        "" => {
            if current_sum > heap.peek().unwrap().0{
                heap.pop();
                heap.push(Reverse(current_sum));
            }
            current_sum = 0;
        }
        _ => current_sum += l.parse::<i32>().unwrap(),
    }});
    heap.drain().map(|i|i.0).sum::<i32>()
}
