use std::collections::HashMap;
use std::hash::Hash;

// Requires a hashmap with all keys having a value
pub fn borda_calc<T: Hash + Eq + Clone> (ballots: &Vec<HashMap<T, u8>>, candidates: &Vec<T>) -> HashMap<T, u8> {
    let mut results: HashMap<T, u8> = candidates.iter().cloned().map(|x| (x, 0)).collect();
    for ballot in ballots.iter() {
        for (key, value) in ballot.iter() {
            let score = results.entry(key.to_owned()).or_insert(0);
            *score += ballot.len() as u8 - value.to_owned();
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
    fn borda_basic() {
        let candidates = vec![Candidate::Washington, Candidate::Jefferson, Candidate::Lincoln, Candidate::Roosevelt];
        let ballot1: HashMap<Candidate, u8> = [(Candidate::Washington, 1),
                                               (Candidate::Jefferson, 4),
                                               (Candidate::Lincoln, 2),
                                               (Candidate::Roosevelt, 3)]
                                               .iter().cloned().collect();
        let ballot2: HashMap<Candidate, u8> = [(Candidate::Washington, 1),
                                               (Candidate::Jefferson, 2),
                                               (Candidate::Lincoln, 3),
                                               (Candidate::Roosevelt, 4)]
                                               .iter().cloned().collect();
        let votes = vec![ballot1, ballot2];
        let expected_result: HashMap<Candidate, u8> = [(Candidate::Washington, 6),
                                               (Candidate::Jefferson, 2),
                                               (Candidate::Lincoln, 3),
                                               (Candidate::Roosevelt, 1)]
                                               .iter().cloned().collect();
        let result = borda_calc(&votes, &candidates);
        assert_eq!(result, expected_result);
    }
}
