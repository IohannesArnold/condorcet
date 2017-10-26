use std::collections::HashMap;
use std::hash::Hash;

pub fn approval_calc<T: Hash + Eq + Clone> (ballots: &Vec<HashMap<T, u8>>, candidates: &Vec<T>) -> HashMap<T, u8> {
    let mut results: HashMap<T, u8> = candidates.iter().cloned().map(|x| (x, 0)).collect();
    for ballot in ballots.iter() {
        for (key, value) in ballot.iter() {
            let count = results.entry(key.to_owned()).or_insert(0);
            *count += value.to_owned();
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, Eq, PartialEq, Clone, Debug)]
    enum Candidate {
        Washington,
        Jefferson,
        Lincoln,
        Roosevelt
    }
    
    #[test]
    fn approval_basic() {
        let candidates = vec![Candidate::Washington, Candidate::Jefferson, Candidate::Lincoln, Candidate::Roosevelt];
        let ballot1: HashMap<Candidate, u8> = [(Candidate::Washington, 1),
                                               (Candidate::Roosevelt, 0)]
                                               .iter().cloned().collect();
        let ballot2: HashMap<Candidate, u8> = [(Candidate::Washington, 1),
                                               (Candidate::Jefferson, 1),
                                               (Candidate::Lincoln, 0),
                                               (Candidate::Roosevelt, 1)]
                                               .iter().cloned().collect();
        let votes = vec![ballot1, ballot2];
        let expected_result: HashMap<Candidate, u8> = [(Candidate::Washington, 2),
                                               (Candidate::Jefferson, 1),
                                               (Candidate::Lincoln, 0),
                                               (Candidate::Roosevelt, 1)]
                                               .iter().cloned().collect();
        let result = approval_calc(&votes, &candidates);
        assert_eq!(result, expected_result);
    }
}
