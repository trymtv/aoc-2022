fn play_score(play: &str) -> i32 {
    match play {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!(),
    }
}

fn round_score(opponent: &str, play: &str) -> i32 {
    match (opponent, play) {
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        _ => 0,
    }
}

#[aoc(day2, part1, Chars)]
pub fn part1(input: &str) -> i32 {
    input.lines().into_iter().fold(0, |sum, l| {
        let mut plays = l.split_whitespace();
        let opponent = plays.next().unwrap();
        let play = plays.next().unwrap();
        sum + round_score(opponent, play) + play_score(play)
    })
}

fn required_play<'life>(opponent: &'life str, required_outcome: &'life str) -> &'life str {
    match required_outcome {
        "X" => match opponent {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            _ => unreachable!(),
        },

        "Y" => match opponent {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            _ => unreachable!(),
        },
        "Z" => match opponent {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

#[aoc(day2, part2, Chars)]
pub fn part2(input: &str) -> i32 {
    input.lines().into_iter().fold(0, |sum, l| {
        let mut plays = l.split_whitespace();
        let opponent = plays.next().unwrap();
        let required_outcome = plays.next().unwrap();
        let play = required_play(opponent, required_outcome);
        sum + round_score(opponent, play) + play_score(play)
    })
}
