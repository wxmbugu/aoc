use std::fmt::Display;
use std::{fs::File, io::Read, str::FromStr};
#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum GamePlay {
    Rock,
    Paper,
    Scissors,
}

#[allow(unused_must_use)]
pub fn rock_paper_scissors(file: String) -> i32 {
    let mut f = File::open(file).expect("file");
    let mut s = String::new();
    f.read_to_string(&mut s);
    let newstring = s.replace(' ', ",");
    let data: Vec<&str> = newstring.split('\n').filter(|x| !x.is_empty()).collect();
    let mut sum = 0;
    for game_play in data {
        let dataset: Vec<&str> = game_play.split('\n').collect();
        for score in dataset {
            let data: Vec<&str> = score.split(',').collect();
            let opponent = match_opponent(data[0].to_string());
            let player = match_player(data[1].to_string());
            let opponent = opponent.unwrap();
            let player = player.unwrap();
            let x = total_score(opponent.to_string(), player.to_string());
            sum += x
        }
    }
    sum
}

fn match_opponent(data: String) -> Option<GamePlay> {
    match data.as_ref() {
        "A" => Some(GamePlay::Rock),
        "B" => Some(GamePlay::Paper),
        "C" => Some(GamePlay::Scissors),
        _ => None,
    }
}

fn match_player(data: String) -> Option<GamePlay> {
    match data.as_ref() {
        "X" => Some(GamePlay::Rock),
        "Y" => Some(GamePlay::Paper),
        "Z" => Some(GamePlay::Scissors),
        _ => None,
    }
}

impl Display for GamePlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for GamePlay {
    type Err = ();
    fn from_str(input: &str) -> Result<GamePlay, Self::Err> {
        match input {
            "Rock" => Ok(GamePlay::Rock),
            "Paper" => Ok(GamePlay::Paper),
            "Scissors" => Ok(GamePlay::Scissors),
            _ => Err(()),
        }
    }
}

fn play_hand(own_hand: GamePlay, other_hand: GamePlay) -> i32 {
    match (&own_hand, &other_hand) {
        // winning play
        _ if own_hand.beats().to_string() == other_hand.to_string() => {
            own_hand.score_gameplay() + 6
        }
        // draw play
        _ if other_hand.to_string() == own_hand.to_string() => own_hand.score_gameplay() + 3,
        // loosing play
        _ => own_hand.score_gameplay(),
    }
}

fn total_score(opponent: String, player: String) -> i32 {
    play_hand(
        GamePlay::from_str(player.as_ref()).unwrap(),
        GamePlay::from_str(opponent.as_ref()).unwrap(),
    )
}

impl GamePlay {
    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
            Self::Paper => Self::Rock,
        }
    }

    fn score_gameplay(self) -> i32 {
        match self {
            GamePlay::Rock => 1,
            GamePlay::Paper => 2,
            GamePlay::Scissors => 3,
        }
    }
}