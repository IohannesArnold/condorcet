extern crate condorcet_primitives;
use condorcet_primitives::*;

use std::collections::HashMap;
use std::hash::Hash;

pub type Ballots<T: Hash + Eq + Clone> = Vec<HashMap<T, u8>>;

#[derive(Debug, PartialEq, Eq)]
pub enum ElectionMethod {
    FirstPastThePost,
    Approval,
    BordaCount,
}

#[derive(Debug, PartialEq, Eq)]
enum ElectionResult<T: Hash + Eq + Clone> {
    HashResult(HashMap<T, u8>)
}

pub struct Election<T: Hash + Eq + Clone> {
    election_type: ElectionMethod,
    winners: u8,
    candidates: Vec<T>,
    candidates_lock: bool,
    ballots: Ballots<T>,
    ballots_lock: bool,
    result: Option<ElectionResult<T>>
}

impl<T: Hash + Eq + Clone> Election<T> {

    pub fn new(election_type: ElectionMethod) -> Election<T> {
        Election {
            election_type: election_type,
            winners: 1,
            candidates: Vec::new(),
            candidates_lock: false,
            ballots: Vec::new(),
            ballots_lock: false,
            result: None,
        }
    }

    pub fn set_winners(&mut self, num: u8) {
        if self.candidates_lock && num > self.candidates.len() as u8 {
            panic!("Attempting to set the number of winners higher than the number of candidates")
        }
        self.winners = num;
    }

    pub fn add_candidate(&mut self, candidate: &T) {
        if self.candidates.contains(candidate) {
            panic!("Attempting to add the same candidate twice")
        }
        if self.candidates_lock {
            panic!("Attempting to add candidate when candidates are locked")
        }
        self.candidates.push(candidate.clone())
    }

    pub fn lock_candidates(&mut self) {
        if self.winners > self.candidates.len() as u8 {
            panic!("Attempting to set the number of candidates lower than the number of winners")
        }
        self.candidates_lock = true
    }

    pub fn add_ballot(&mut self, ballot: &HashMap<T, u8>) {
        if self.ballots_lock {
            panic!("Attempting to add ballot when ballots are locked")
        }
        for (key, val) in ballot.iter() {
            if self.candidates_lock && !self.candidates.contains(key) {
                panic!("Ballot contains unauthorized candidate")
            }
        }
        self.ballots.push(ballot.clone())
    }

    pub fn lock_ballots(&mut self) {
        self.ballots_lock = true
    }

    pub fn calculate(&mut self) {
        if !self.candidates_lock { self.lock_candidates() }
        if !self.ballots_lock { self.lock_ballots() }
        if !(self.result == None) {
            panic!("Trying to calculate election that already has results")
        }
        self.result = match self.election_type {
            ElectionMethod::BordaCount => Some(ElectionResult::HashResult(borda_calc(&self.ballots, &self.candidates))),
            ElectionMethod::FirstPastThePost => Some(ElectionResult::HashResult(first_past_the_post_calc(&self.ballots, &self.candidates))),
            ElectionMethod::Approval =>Some(ElectionResult::HashResult(approval_calc(&self.ballots, &self.candidates)))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let test_election: Election<u32> = Election::new(ElectionMethod::Approval);
        assert_eq!(test_election.election_type, ElectionMethod::Approval);
        assert_eq!(test_election.winners, 1);
        assert_eq!(test_election.candidates.len(), 0);
        assert_eq!(test_election.candidates_lock, false);
        assert_eq!(test_election.ballots.len(), 0);
        assert_eq!(test_election.ballots_lock, false);
        assert_eq!(test_election.result, None);
    }

    #[test]
    fn test_winners() {
        let mut test_election: Election<u32> = Election::new(ElectionMethod::Approval);
        test_election.set_winners(5);
        assert_eq!(test_election.winners, 5);
    }
}
