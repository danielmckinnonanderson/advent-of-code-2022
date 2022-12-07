use std::io;
use std::io::prelude::*;
use std::str;


pub fn solve_part_1() {
    let input = parse_input();

    let mut cumulative_score = 0;

    for round in input {
        let round: (Move, Move) = parse_strategy_guide_into_moves(&round);
        let outcome = determine_outcome(&round);

        let score = score_round(&round.1, &outcome);
        cumulative_score += score;
    }

    println!("{}", cumulative_score);
}

pub fn solve_part_2() {
    let input = parse_input();

    let mut cumulative_score = 0;

    for round in input {
        let (opponent_move, desired_outcome) = parse_strategy_guide_into_move_and_outcome(&round);
        let self_move = determine_move_from_desired_outcome(&opponent_move, &desired_outcome);

        let score = score_round(&self_move, &desired_outcome);
        cumulative_score += score;
    }

    println!("{}", cumulative_score);
}

pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    // scoring
    // Rock = 1
    // Paper = 2
    // Scissors = 3
    pub fn value(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    // Loss = 0
    // Draw = 3
    // Win = 6
    fn value(&self) -> i32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

pub fn parse_input() -> Vec<String> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let vec = stdin_lock.lines().filter_map(|l| l.ok()).collect();

    vec
}

// strategy guide
// A = opponent rock
// B = opponent paper
// C = opponent scissors
// X = self rock
// Y = self paper
// Z = self scissors
pub fn parse_strategy_guide_into_moves(line: &str) -> (Move, Move) {
    let opponent_move = line.chars().nth(0).unwrap();
    let self_move = line.chars().nth(2).unwrap();

    let opponent_move = match opponent_move {
        '\x41' => Move::Rock,
        '\x42' => Move::Paper,
        '\x43' => Move::Scissors,
        _ => panic!("Invalid input character! Character was {}", opponent_move),
    };
    let self_move = match line.chars().nth(2).unwrap() {
        '\x58' => Move::Rock,
        '\x59' => Move::Paper,
        '\x5A' => Move::Scissors,
        _ => panic!("Invalid input character! Character was {}", self_move),
    };

    (opponent_move, self_move)
}

pub fn parse_strategy_guide_into_move_and_outcome(line: &str) -> (Move, Outcome) {
    let opponent_move = line.chars().nth(0).unwrap();
    let desired_outcome = line.chars().nth(2).unwrap();

    let opponent_move = match opponent_move {
        '\x41' => Move::Rock,
        '\x42' => Move::Paper,
        '\x43' => Move::Scissors,
        _ => panic!("Invalid input character! Character was {}", opponent_move),
    };
    let desired_outcome = match line.chars().nth(2).unwrap() {
        '\x58' => Outcome::Loss,
        '\x59' => Outcome::Draw,
        '\x5A' => Outcome::Win,
        _ => panic!("Invalid input character! Character was {}", desired_outcome),
    };

    (opponent_move, desired_outcome)
}

pub fn determine_outcome(round: &(Move, Move)) -> Outcome {
    let (opponent_move, self_move) = round;

    match self_move {
        Move::Rock => match opponent_move {
            Move::Rock => Outcome::Draw,
            Move::Paper => Outcome::Loss,
            Move::Scissors => Outcome::Win,
        },
        Move::Paper => match opponent_move {
            Move::Rock => Outcome::Win,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Loss,
        },
        Move::Scissors => match opponent_move {
            Move::Rock => Outcome::Loss,
            Move::Paper => Outcome::Win,
            Move::Scissors => Outcome::Draw,
        },
    }
}

pub fn determine_move_from_desired_outcome(
    opponent_move: &Move,
    desired_outcome: &Outcome,
) -> Move {
    match opponent_move {
        Move::Rock => match desired_outcome {
            Outcome::Loss => Move::Scissors,
            Outcome::Draw => Move::Rock,
            Outcome::Win => Move::Paper,
        },
        Move::Paper => match desired_outcome {
            Outcome::Loss => Move::Rock,
            Outcome::Draw => Move::Paper,
            Outcome::Win => Move::Scissors,
        },
        Move::Scissors => match desired_outcome {
            Outcome::Loss => Move::Paper,
            Outcome::Draw => Move::Scissors,
            Outcome::Win => Move::Rock,
        },
    }
}

pub fn score_round(self_move: &Move, outcome: &Outcome) -> i32 {
    self_move.value() + outcome.value()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn scores_rounds_correctly() {
        let self_move = Move::Rock;
        let outcome = Outcome::Win;
        let result = score_round(&self_move, &outcome);

        assert_eq!(7, result);
    }

    #[test]
    fn scores_sci_loss_correctly() {
        let self_move = Move::Scissors;
        let outcome = Outcome::Loss;
        let result = score_round(&self_move, &outcome);

        assert_eq!(3, result);
    }

    #[test]
    fn parses_inputs_correctly() {
        let input = String::from("A Y");
        let result = parse_strategy_guide_into_moves(&input);
        assert_eq!(1, result.0.value());
        assert_eq!(2, result.1.value());

        let input = String::from("B X");
        let result = parse_strategy_guide_into_moves(&input);
        assert_eq!(2, result.0.value());
        assert_eq!(1, result.1.value());

        let input = String::from("C Z");
        let result = parse_strategy_guide_into_moves(&input);
        assert_eq!(3, result.0.value());
        assert_eq!(3, result.1.value());
    }

    #[test]
    fn determines_outcome_rock_vs_sci() {
        let s = Move::Rock;
        let o = Move::Scissors;
        let result = determine_outcome(&(s, o));

        assert_eq!(0, result.value());
    }

    #[test]
    fn determines_outcome_rock_vs_paper() {
        let s = Move::Rock;
        let o = Move::Paper;
        let result = determine_outcome(&(s, o));

        assert_eq!(6, result.value());
    }

    #[test]
    fn determines_outcome_rock_vs_roc() {
        let s = Move::Rock;
        let o = Move::Rock;
        let result = determine_outcome(&(s, o));

        assert_eq!(3, result.value());
    }

    #[test]
    fn blackbox_test_part_1() {
        let input = "A Z";
        let parsed_input: (Move, Move) = parse_strategy_guide_into_moves(&input);
        let outcome: Outcome = determine_outcome(&parsed_input);
        let score = score_round(&parsed_input.1, &outcome);

        assert_eq!(3, score);

        let input = "B Y";
        let parsed_input: (Move, Move) = parse_strategy_guide_into_moves(&input);
        let outcome: Outcome = determine_outcome(&parsed_input);
        let score = score_round(&parsed_input.1, &outcome);

        assert_eq!(5, score);

        let input = "B Z";
        let parsed_input: (Move, Move) = parse_strategy_guide_into_moves(&input);
        let outcome: Outcome = determine_outcome(&parsed_input);
        let score = score_round(&parsed_input.1, &outcome);

        assert_eq!(9, score);
    }
}
