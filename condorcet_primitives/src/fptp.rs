use std::collections::HashMap;
use std::hash::Hash;

pub fn first_past_the_post_calc<T: Hash + Eq + Clone> (ballots: &Vec<HashMap<T, u8>>, candidates: &Vec<T>) -> HashMap<T, u8> {
    let mut results: HashMap<T, u8> = candidates.iter().cloned().map(|x| (x, 0)).collect();
    for ballot in ballots.iter() {
        for (key, value) in ballot.iter() {
            let count = results.entry(key.to_owned()).or_insert(0);
            *count += value.to_owned();
        }
    }
    results
}

/*
 * Commented out because these tests reflect an earlier type definition
 * for the ballot
 
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
    fn fptp_with_bools() {
        let candidates = vec![true, false];
        let votes = vec![true, false, true, false, true];
        let mut expected_result = HashMap::new();
        expected_result.insert(true, 3);
        expected_result.insert(false, 2);
        let result = first_past_the_post_calc(&votes, &candidates);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn fptp_with_ints() {
        let candidates = vec![1, 2, 3, 42];
        let votes = vec![1, 2, 1, 3, 42, 42, 42];
        let mut expected_result = HashMap::new();
        expected_result.insert(1, 2);
        expected_result.insert(2, 1);
        expected_result.insert(3, 1);
        expected_result.insert(42, 3);
        let result = first_past_the_post_calc(&votes, &candidates);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn fptp_with_strings() {
        let candidates = vec!["Washington", "Lincoln", "Kennedy"];
        let votes = vec!["Washington", "Washington", "Lincoln", "Washington", "Lincoln", "Kennedy"];
        let mut expected_result = HashMap::new();
        expected_result.insert("Washington", 3);
        expected_result.insert("Lincoln", 2);
        expected_result.insert("Kennedy", 1);
        let result = first_past_the_post_calc(&votes, &candidates);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn fptp_with_strings_and_unsucessful_candidate() {
        let candidates = vec!["Washington", "Lincoln", "Kennedy"];
        let votes = vec!["Washington", "Washington", "Lincoln", "Washington", "Lincoln"]; 
        let mut expected_result = HashMap::new();
        expected_result.insert("Washington", 3);
        expected_result.insert("Lincoln", 2);
        expected_result.insert("Kennedy", 0);
        let result = first_past_the_post_calc(&votes, &candidates);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn fptp_with_enums() {
        let candidates = vec![Candidate::Washington, Candidate::Jefferson, Candidate::Lincoln, Candidate::Roosevelt];
        let votes = vec![Candidate::Washington, Candidate::Washington, Candidate::Lincoln,Candidate::Washington, Candidate::Lincoln, Candidate::Roosevelt];
        let mut expected_result = HashMap::new();
        expected_result.insert(Candidate::Washington, 3);
        expected_result.insert(Candidate::Jefferson, 0);
        expected_result.insert(Candidate::Lincoln, 2);
        expected_result.insert(Candidate::Roosevelt, 1);
        let result = first_past_the_post_calc(&votes, &candidates);
        assert_eq!(result, expected_result);
    }
} */
