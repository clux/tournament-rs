use std::fmt;
use std::cmp::Ordering;

//use super::TRes;

// TODO: add sanity checks that Seed > 0 from user input
const INVALID: i32 = -1;
const PLACEHOLDER: i32 = 0;
pub type Seed = i32;


pub type Score = u32;

/// A unique representation of a match in the tournament
#[derive(Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct MatchId {
    /// Bracket/group/section of the match
    #[serde(rename = "s")]
    pub section: u32,
    /// Round number in this section
    #[serde(rename = "r")]
    pub round: u32,
    /// Match number in this round
    ///
    /// Note that match is a keyword, so using game internally, but avoid using `g`
    /// because that can be confused with group numbers in group stages.
    #[serde(rename = "m")]
    pub game: u32,
}

impl fmt::Display for MatchId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S{}R{}M{}", self.section, self.round, self.game)
    }
}

impl MatchId {
    pub fn new(section: u32, round: u32, game: u32) -> MatchId {
        MatchId {
            section,
            round,
            game,
        }
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
    pub fn is_ready(&self) -> bool {
        self.players.iter().all(|p| *p > PLACEHOLDER)
    }

    pub fn contains(&self, player: Seed) -> bool {
        self.players.iter().any(|p| *p == player)
    }

    pub fn is_scored(&self) -> bool {
        self.scores.is_some()
    }
}

pub trait Base<E> {
    // TODO: need to get ALL matches mutably for score (used in defaults)
    /// Get a reference to the matches
    fn get_matches(&self) -> &Vec<Match>;

    /// Score validator that takes generic error
    fn validate(&self, _: &Match, _: &Vec<Score>) -> Result<(), E> {
        Ok(())
    }

    /// Check if a match is safe to rescore at this time
    fn rescorable(&self, _: &Match) -> bool {
        true
    }

    /// Whether a tournament is complete (some can finish early by overriding this)
    fn finished(&self) -> bool {
        self.get_matches().iter().all(|m| m.scores.is_some())
    }


    // Initialilze a tourament TODO: needs generic options struct
}

// TODO: implement a helper trait for structs that impl Base
// - this needs to create `score`, `results`, etc..
pub trait Tournament<E> {
    fn find(&self, id: MatchId) -> Option<&Match>;
    fn score(&mut self, id: MatchId, score: &Vec<Score>) -> Result<(), E>;
}
impl<T, E> Tournament<E> for T
where
    T: Base<E>,
{
    fn find(&self, id: MatchId) -> Option<&Match> {
        self.get_matches().iter().find(|m| m.id == id)
    }
    fn score(&mut self, id: MatchId, score: &Vec<Score>) -> Result<(), E> {
        if let Some(m) = self.find(id) {
            if !m.is_ready() {
                // return Err - missing players
            }
            if score.len() != m.players.len() {
                // return Err - invalid score array length
            }
            // TODO: allow this to be overridden
            if m.is_scored() && !self.rescorable(m) {
                // return Err - cannot rescore past matches
            }
            self.validate(m, score)
        } else {
            // return Err - matchid not valid in tournament
            unimplemented!();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization() {
        let id = MatchId::new(1, 2, 3);
        assert_eq!(
            "{\"s\":1,\"r\":2,\"m\":3}",
            serde_json::to_string(&id).unwrap()
        );

        let wo: Seed = INVALID;
        let womark = serde_json::to_string(&wo).unwrap();
        assert_eq!(womark, "-1");
        let ph: Seed = PLACEHOLDER;
        let placeholder = serde_json::to_string(&ph).unwrap();
        assert_eq!(placeholder, "0");
        let p2: Seed = 2;
        let player2 = serde_json::to_string(&p2).unwrap();
        assert_eq!(player2, "2");
    }
    #[test]
    fn ordering() {
        // lowest match id's by order can be played first
        let w23 = MatchId::new(1, 2, 3);
        let w41 = MatchId::new(1, 4, 1);
        assert!(w23 < w41);
        let l11 = MatchId::new(2, 1, 1);
        assert!(w23 < l11);
        let l21 = MatchId::new(2, 2, 1);
        assert!(l21 > l11);

        // NB: Ord for double eliminations Duel works because grand finals are in LB
        // it's a non-intuitive, but working order that allows scoring in order
        // anything more complicated wouldn't be easily sortable
        // but could be overridden perhaps
        // if we want this, then we need MatchId's factored into different impls
    }

}
