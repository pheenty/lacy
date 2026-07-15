use std::{collections::HashMap, iter};

pub type Score = u16;

const SCORE_PATTERN_FULL_MATCH: Score = 60;
const SCORE_PATTERN_CONTAINED: Score = 10;
const SCORE_CHAR_STARTING: Score = 5;
const SCORE_CHAR_HIT: Score = 1;
const PENALTY_CHAR_MISS: Score = 5;

/// Creates a score of how much the input and the pattern match
/// The higher the score the better. There is no max score.
pub fn fuzzy_match_score(input: &str, pattern: &str) -> Score {
    let input = input.to_lowercase();
    let pattern = pattern.to_lowercase();

    let whole_pattern_score = {
        if input == pattern {
            SCORE_PATTERN_FULL_MATCH
        } else if input.contains(&pattern) {
            SCORE_PATTERN_CONTAINED
        } else {
            0
        }
    };

    let starting_score = SCORE_CHAR_STARTING
        * iter::zip(input.chars(), pattern.chars())
            .take_while(|(inp, pat)| inp == pat)
            .count() as Score;

    let (char_occurrence_score, char_miss_penalty) = {
        let input = freqmap(&input);
        let (hits, misses) = freqmap(&pattern)
            .iter()
            .map(|(char, &amt_in_pat)| {
                let amt_in_inp = input.get(char).copied().unwrap_or_default();
                let hits = amt_in_pat.min(amt_in_inp);
                (hits, amt_in_pat - hits)
            })
            .reduce(|(hits_acc, misses_acc), (hits, misses)| (hits_acc + hits, misses_acc + misses))
            .unwrap_or_default();

        (
            SCORE_CHAR_HIT * hits as Score,
            PENALTY_CHAR_MISS * misses as Score,
        )
    };

    (whole_pattern_score + starting_score + char_occurrence_score).saturating_sub(char_miss_penalty)
}

fn freqmap(str: &str) -> HashMap<char, u8> {
    let mut map = HashMap::with_capacity(str.len());
    for char in str.chars() {
        *map.entry(char).or_default() += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::fuzzy_match_score as score;

    #[test]
    fn test_simple() {
        assert_eq!(score("test", "test"), score("test", "test"));
        assert_eq!(score("test", "uoa"), 0);

        assert!(score("test", "test") > score("test", "tes"));
        assert!(score("ttest", "tt") > score("ttest", "t"));
    }

    #[test]
    fn test_advanced() {
        assert!(score("helloworld", "world") > score("helloworld", "elwo"));
        assert!(score("helloworld", "hello") > score("helloworld", "hellohello"));
    }

    #[test]
    fn test_starting_with() {
        assert!(score("test", "t") > score("test", "tt"));
        assert!(score("test-abc", "te") > score("test-abc", "ta"));
        assert!(score("test abc", "te") > score("test abc", "ta"));
        assert!(score("test_abc", "te") > score("test_abc", "ta"));

        assert!(score("test_abc_a", "te") > score("test_abc_a", "taa"));
        assert!(score("test_abc_a", "tea") > score("test_abc_a", "taa"));

        assert!(score("testAbc", "te") > score("testAbc", "tA"));
    }

    #[test]
    fn test_negative_queries() {
        assert!(score("helloworld", "ellovvvv") == 0);
        assert!(score("helloworld", "ww") == 0);
    }
}
