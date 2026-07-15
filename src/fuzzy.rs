use std::{collections::HashMap, iter};

const SCORE_PATTERN_FULL_MATCH: i32 = 60;
const SCORE_PATTERN_CONTAINS: i32 = 10;

const SCORE_STARTING_CHAR: i32 = 5;

const SCORE_CHARS_HIT: i32 = 1;
const SCORE_CHARS_MISS: i32 = -5;

/// Creates a score of how much the input and the pattern match
/// The higher the score the better. There is no max score.
pub fn fuzzy_match_score(input: &str, pattern: &str) -> i32 {
    let input = input.to_lowercase();
    let pattern = pattern.to_lowercase();

    let whole_pattern_score = {
        if input == pattern {
            SCORE_PATTERN_FULL_MATCH
        } else if input.contains(&pattern) {
            SCORE_PATTERN_CONTAINS
        } else {
            0
        }
    };

    let starting_score = SCORE_STARTING_CHAR
        * iter::zip(input.chars(), pattern.chars())
            .take_while(|(inp, pat)| inp == pat)
            .count() as i32;

    let char_occurrence_score = {
        let input = freqmap(&input);
        freqmap(&pattern)
            .iter()
            .map(|(char, amt_in_pat)| {
                let amt_in_inp = input.get(char).copied().unwrap_or_default();

                let hits = amt_in_pat.min(&amt_in_inp);
                let misses = amt_in_pat - hits;

                SCORE_CHARS_HIT * hits + SCORE_CHARS_MISS * misses
            })
            .sum::<i32>()
    };

    (whole_pattern_score + starting_score + char_occurrence_score).max(0)
}

fn freqmap(str: &str) -> HashMap<char, i32> {
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
