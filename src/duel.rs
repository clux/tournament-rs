use std::fmt;

use super::game::{Match, MatchId, Base, Score};

#[derive(Debug)]
pub enum DuelErr {
    ScoreLength,
    IllegalDraw,
}
impl fmt::Display for DuelErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DuelErr::ScoreLength => write!(f, "Invalid score array length != 2"),
            DuelErr::IllegalDraw => write!(f, "Cannot draw a duel match"),
        }
    }
}

pub struct Duel {
    matches: Vec<Match>,
}
impl Base<DuelErr> for Duel {
    fn get_matches(&self) -> &Vec<Match> {
        &self.matches
    }

    fn validate(&self, _: &Match, score: &Vec<Score>) -> Result<(), DuelErr> {
        if score.len() != 2 {
            Err(DuelErr::ScoreLength)
        } else if score[0] == score[1] {
            Err(DuelErr::IllegalDraw)
        } else {
            Ok(())
        }
    }
    // TODO: custom look at GF if underdow won or not
    //fn finished(&self) -> bool {
    //
    //}
}
