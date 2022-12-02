use crate::day2::Move::{Paper, Rock, Scissors};
use crate::day2::Outcome::{Draw, Lose, Win};
use crate::Error::InputError;
use crate::Result;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

// Returns if `b` beats `a`
fn outcome(a: Move, b: Move) -> Outcome {
    match (a, b) {
        (Rock, Paper) => Win,
        (Rock, Scissors) => Lose,
        (Paper, Scissors) => Win,
        (Paper, Rock) => Lose,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Lose,
        (a, b) if a == b => Draw,
        _ => unreachable!(),
    }
}

fn required_move(a: Move, outcome: Outcome) -> Move {
    match (a, outcome) {
        (Rock, Win) => Paper,
        (Rock, Lose) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Lose) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Lose) => Paper,
        (a, Draw) => a,
    }
}

fn parse_move(input: &str) -> Result<Move> {
    match input {
        "A" => Ok(Rock),
        "B" => Ok(Paper),
        "C" => Ok(Scissors),
        "X" => Ok(Rock),
        "Y" => Ok(Paper),
        "Z" => Ok(Scissors),
        _ => Err(InputError),
    }
}

fn parse_outcome(input: &str) -> Result<Outcome> {
    match input {
        "X" => Ok(Lose),
        "Y" => Ok(Draw),
        "Z" => Ok(Win),
        _ => Err(InputError),
    }
}

fn score_from_move(mov: Move) -> u32 {
    match mov {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn score_outcome(outcome: Outcome) -> u32 {
    match outcome {
        Win => 6,
        Draw => 3,
        Lose => 0,
    }
}

fn evaluate_line(line: &str) -> Result<u32> {
    let chunks: Vec<&str> = line.split(" ").collect();
    if chunks.len() != 2 {
        return Err(InputError);
    }
    let move_1 = parse_move(chunks[0])?;
    let move_2 = parse_move(chunks[1])?;
    let outcome = outcome(move_1, move_2);
    Ok(score_from_move(move_2) + score_outcome(outcome))
}

fn evaluate_line_part_two(line: &str) -> Result<u32> {
    let chunks: Vec<&str> = line.split(" ").collect();
    if chunks.len() != 2 {
        return Err(InputError);
    }
    let move_1 = parse_move(chunks[0])?;
    let outcome = parse_outcome(chunks[1])?;
    let move_2 = required_move(move_1, outcome);

    Ok(score_from_move(move_2) + score_outcome(outcome))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::{read_lines, split_lines};

    #[test]
    fn part_one_small() {
        let input = "A Y
B X
C Z";

        let lines = split_lines(input);
        let outcomes: Vec<u32> = lines
            .iter()
            .filter_map(|l| evaluate_line(l.as_str()).ok())
            .collect();
        assert_eq!(outcomes, [8, 1, 6]);

        assert_eq!(outcomes.iter().sum::<u32>(), 15);
    }

    #[test]
    fn part_one() {
        let input: Vec<String> = read_lines("input/day2-1")
            .unwrap()
            .filter_map(|s| s.ok())
            .collect();

        let outcomes: Vec<u32> = input
            .iter()
            .filter_map(|l| evaluate_line(l.as_str()).ok())
            .collect();
        assert_eq!(outcomes.iter().sum::<u32>(), 9177);
    }

    #[test]
    fn part_two_small() {
        let input = "A Y
B X
C Z";

        let lines = split_lines(input);
        let outcomes: Vec<u32> = lines
            .iter()
            .filter_map(|l| evaluate_line_part_two(l.as_str()).ok())
            .collect();
        assert_eq!(outcomes, [4, 1, 7]);

        assert_eq!(outcomes.iter().sum::<u32>(), 12);
    }

    #[test]
    fn part_two() {
        let input: Vec<String> = read_lines("input/day2-1")
            .unwrap()
            .filter_map(|s| s.ok())
            .collect();

        let outcomes: Vec<u32> = input
            .iter()
            .filter_map(|l| evaluate_line_part_two(l.as_str()).ok())
            .collect();
        assert_eq!(outcomes.iter().sum::<u32>(), 12111);
    }
}
