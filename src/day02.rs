use std::fs::File;
use std::io::{BufRead, BufReader};

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;
const LOSE_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn parse_choice(input: char) -> Choice {
    match input {
        'A' => Choice::Rock,
        'X' => Choice::Rock,
        'B' => Choice::Paper,
        'Y' => Choice::Paper,
        'C' => Choice::Scissors,
        'Z' => Choice::Scissors,
        _ => panic!(),
    }
}

fn parse_outcome(input: char) -> Outcome {
    match input {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

struct Game {
    them: Choice,
    us: Choice,
}

struct Part2Game {
    them: Choice,
    outcome: Outcome,
}

pub trait Score {
    fn our_score(&self) -> u32;
    fn game_score(&self) -> u32;
}

impl Score for Game {
    fn our_score(&self) -> u32 {
        match self.us {
            Choice::Rock => ROCK_SCORE,
            Choice::Paper => PAPER_SCORE,
            Choice::Scissors => SCISSORS_SCORE,
        }
    }

    fn game_score(&self) -> u32 {
        if (matches!(self.us, Choice::Rock) && matches!(self.them, Choice::Scissors))
            || (matches!(self.us, Choice::Scissors) && matches!(self.them, Choice::Paper))
            || (matches!(self.us, Choice::Paper) && matches!(self.them, Choice::Rock))
        {
            WIN_SCORE
        } else if (matches!(self.us, Choice::Rock) && matches!(self.them, Choice::Rock))
            || (matches!(self.us, Choice::Scissors) && matches!(self.them, Choice::Scissors))
            || (matches!(self.us, Choice::Paper) && matches!(self.them, Choice::Paper))
        {
            DRAW_SCORE
        } else {
            LOSE_SCORE
        }
    }
}

impl Score for Part2Game {
    fn our_score(&self) -> u32 {
        if (matches!(self.outcome, Outcome::Lose) && matches!(self.them, Choice::Scissors))
            || (matches!(self.outcome, Outcome::Draw) && matches!(self.them, Choice::Paper))
            || (matches!(self.outcome, Outcome::Win) && matches!(self.them, Choice::Rock))
        {
            PAPER_SCORE
        } else if (matches!(self.outcome, Outcome::Lose) && matches!(self.them, Choice::Rock))
            || (matches!(self.outcome, Outcome::Draw) && matches!(self.them, Choice::Scissors))
            || (matches!(self.outcome, Outcome::Win) && matches!(self.them, Choice::Paper))
        {
            SCISSORS_SCORE
        } else {
            ROCK_SCORE
        }
    }

    fn game_score(&self) -> u32 {
        match self.outcome {
            Outcome::Lose => LOSE_SCORE,
            Outcome::Draw => DRAW_SCORE,
            Outcome::Win => WIN_SCORE,
        }
    }
}

pub(crate) fn day02() {
    let f: File = File::open("data/day02.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let mut games: Vec<Game> = vec![];
    let mut part_2_games: Vec<Part2Game> = vec![];

    for line in reader.lines() {
        let game = line.unwrap();
        let them = game.chars().next().unwrap();
        let us = game.chars().last().unwrap();
        games.push(Game {
            them: parse_choice(them),
            us: parse_choice(us),
        });
        part_2_games.push(Part2Game {
            them: parse_choice(them),
            outcome: parse_outcome(us),
        });
    }

    println!(
        "Score for part 1 is {}",
        games
            .iter()
            .map(|x| x.our_score() + x.game_score())
            .sum::<u32>()
    );

    println!(
        "Score for part 2 is {}",
        part_2_games
            .iter()
            .map(|x| x.our_score() + x.game_score())
            .sum::<u32>()
    );
}
