use crate::utils::get_input;
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for RPS {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<i32> for RPS {
    fn from(value: i32) -> Self {
        match value {
            1 => RPS::Rock,
            2 => RPS::Paper,
            3 => RPS::Scissors,
            _ => panic!(),
        }
    }
}

impl From<RPS> for i32 {
    fn from(value: RPS) -> Self {
        match value {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl From<&RPS> for i32 {
    fn from(value: &RPS) -> Self {
        match value {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<&Outcome> for i32 {
    fn from(value: &Outcome) -> Self {
        match value {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

impl From<(&RPS, &RPS)> for Outcome {
    fn from(value: (&RPS, &RPS)) -> Self {
        match value {
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                Outcome::Draw
            }
            (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) | (RPS::Rock, RPS::Scissors) => {
                Outcome::Win
            }
            _ => Outcome::Lose,
        }
    }
}

pub fn task1() {
    let mut score: u128 = 0;
    for game in get_input(2).lines() {
        let moves = game.split_whitespace().collect::<Vec<&str>>();
        let my_move: RPS = moves[1].into();
        let theirs: RPS = moves[0].into();
        let outcome: Outcome = (&my_move, &theirs).into();
        let outcome: i32 = (&outcome).into();
        let my_move_score: i32 = my_move.into();
        score += my_move_score as u128 + outcome as u128;
    }
    println!("Score: {}", score);
}

pub fn task2() {
    let mut score: u128 = 0;
    for game in get_input(2).lines() {
        let moves = game.split_whitespace().collect::<Vec<&str>>();
        let outcome: Outcome = moves[1].into();
        let theirs: RPS = moves[0].into();
        let outcome_score: i32 = (&outcome).into();
        let move_score: i32 = match outcome {
            Outcome::Draw => theirs,
            Outcome::Lose => {
                let their_num: i32 = theirs.into();
                (((their_num + 1) % 3) + 1).into()
            }
            Outcome::Win => {
                let their_num: i32 = theirs.into();
                (((their_num) % 3) + 1).into()
            }
        }
        .into();
        score += outcome_score as u128 + move_score as u128;
    }
    println!("Score: {}", score);
}
