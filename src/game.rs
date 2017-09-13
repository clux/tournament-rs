use std::fmt;
use std::cmp::Ordering;

use super::TRes;

// TODO: simpler serialize for this enum
#[derive(Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Seed {
    Invalid,
    Placeholder,
    Player(u32),
}
pub type Score = u32;

// TODO: simpler serialize for this enum (shorter keys)
/// A unique and sensible representation of a match in the tournament
// NB: Ord for Duel works because GF are last in LB
// it's a non-intuitive, but working order
// anything more complicated wouldn't be easily sortable
// but could be overridden perhaps
// if we want this, then we need MatchId's factored into different impls
#[derive(Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct MatchId {
    /// Bracket/group/section of the match
    pub section: u32,
    /// Round number in this section
    pub round: u32,
    /// Match number in this round
    pub game: u32,
}

impl fmt::Display for MatchId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S{}R{}G{}", self.section, self.round, self.game)
    }
}

#[derive(Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Match {
    /// A unique representation of the match in the tournament
    pub id: MatchId,
    /// Ordered seeds playing this match
    pub players: Vec<Seed>,
    /// Ordered scores that zip with players (if played)
    pub scores: Option<Vec<Score>>,
    // TODO: extra match data
}

impl Ord for Match {
    fn cmp(&self, other: &Match) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Match {
    pub fn started(&self) -> bool {
        return self.players.iter().all(|p| p > &Seed::Invalid);
    }
}

pub trait Tournament {
    /// Progression function that must return (because validate checked errors)
    fn score(&mut self, id: MatchId, score: &Vec<Score>);
    /// Score validator that takes generic error: TODO: template and optional!
    fn validate(&self, id: MatchId, score: &Vec<Score>) -> TRes<()>;
    /// Whether a function returns early TODO: opitonal (can check all matches by def)
    fn finished(&self) -> bool;
    /// How each tournament searches for matches
    fn find(&self, id: MatchId) -> Match;

    // Initialilze a tourament TODO: needs generic options struct
}

// TODO: implement a helper trait for structs that impl Tournament
//


// example implementation
//struct Duel {
//    matches: Vec<Match>,
//}
//impl Tournament for Duel {
//
//}
