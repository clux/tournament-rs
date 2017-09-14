use super::game::{Match, MatchId, Tournament, Score};
use super::TRes;

// example implementation
struct Duel {
    matches: Vec<Match>,
}
impl Tournament for Duel {
  fn score(&mut self, id: MatchId, score: &Vec<Score>) {
    unimplemented!();
  }
  fn validate(&self, id: MatchId, score: &Vec<Score>) -> TRes<()> {
    unimplemented!();
  }
  fn finished(&self) -> bool {
    self.matches.iter().all(|m| m.scores.is_some())
  }
  fn find(&self, id: MatchId) -> Option<&Match> {
    self.matches.iter().find(|m| m.id == id)
  }
}
