use std::fmt::Display;
use std::{fs::File, io::Read, str::FromStr};
#[derive(Debug)]
enum GamePlay {
    Rock,
    Paper,
    Scissors,
}
enum GameState {
    Win,
    Lose,
    Draw,
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
            let opponent = match_hand(data[0].to_string()).unwrap();
            let player = match_hand(data[1].to_string()).unwrap();
            let x = total_score(opponent.to_string(), player.to_string());
            sum += x
        }
    }
    sum
}

fn match_hand(data: String) -> Option<GamePlay> {
    match data.as_ref() {
        "A" | "X" => Some(GamePlay::Rock),
        "B" | "Y" => Some(GamePlay::Paper),
        "C" | "Z" => Some(GamePlay::Scissors),
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

fn play_hand(own_hand: GamePlay, other_hand: GamePlay) -> (GamePlay, GameState) {
    match (&own_hand, &other_hand) {
        // winning play
        _ if own_hand.beats().to_string() == other_hand.to_string() => {
            (own_hand, GameState::Win)
        }
        // draw play
        _ if other_hand.to_string() == own_hand.to_string() => {
            (own_hand, GameState::Lose)
        }
        // loosing play
        _ => (own_hand, GameState::Lose),
    }
}

fn total_score(opponent: String, player: String) -> i32 {
    let (game_play, game_state) = play_hand(
        GamePlay::from_str(player.as_ref()).unwrap(),
        GamePlay::from_str(opponent.as_ref()).unwrap(),
    );
    game_state.compute(game_play)
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
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl GameState {
    fn compute(self, own_hand: GamePlay) -> i32 {
        match self {
            Self::Win => own_hand.score_gameplay() + 6,
            Self::Draw => own_hand.score_gameplay() + 3,
            Self::Lose => own_hand.score_gameplay(),
        }
    }
}
