//use anyhow::{anyhow, Result};
//use std::error::Error;

// https://leetcode.com/problems/regular-expression-matching/

#[derive(Debug)]
struct Pattern {
    pattern_chars: Vec<char>,
}
impl Pattern {
    fn new(pattern: String) -> Result<Pattern, String> {
        if pattern.len() == 0 {
            Err("Pattern cannot be empty".to_string())
        } else {
            Ok(Pattern {
                pattern_chars: pattern.chars().collect::<Vec<char>>()})
        }
    }

    fn matches(&self, word: String) -> bool {
        let mut pattern_ix = 0;
        //let mut matched = false;
        for char in word.chars() {
            if pattern_ix == self.pattern_chars.len() {
                // We've exhausted the pattern, but not yet the string.
                // the last pattern char was not '*'
                return false;
            }
            match self.pattern_chars[pattern_ix] {
                c if c == char => {
                    pattern_ix = pattern_ix + 1;
                }
                '.' => {
                    pattern_ix = pattern_ix + 1;
                }
                '*' => {
                }
                _ => return false

            }
            //matched = true;
        }
        pattern_ix == self.pattern_chars.len()
    }
}

#[cfg(test)]
mod tests {
    //use crate::zigzag_coder::code;

    use crate::regex::Pattern;

    #[test]
    fn test() {
        assert!(Pattern::new("".to_string()).is_err());
        assert!(Pattern::new("a".to_string()).unwrap().matches("a".to_string()));
        assert!(!Pattern::new("a".to_string()).unwrap().matches("ab".to_string()));
        assert!(!Pattern::new("a".to_string()).unwrap().matches("ba".to_string()));
        assert!(!Pattern::new("a".to_string()).unwrap().matches("aab".to_string()));
        assert!(!Pattern::new("a".to_string()).unwrap().matches("baa".to_string()));

        assert!(Pattern::new("a.".to_string()).unwrap().matches("ab".to_string()));
        assert!(Pattern::new("a..".to_string()).unwrap().matches("abc".to_string()));
        assert!(!Pattern::new("a.".to_string()).unwrap().matches("abc".to_string()));
        assert!(Pattern::new("..c".to_string()).unwrap().matches("abc".to_string()));
        assert!(Pattern::new(".c".to_string()).unwrap().matches("ac".to_string()));
        assert!(!Pattern::new("..c".to_string()).unwrap().matches("ac".to_string()));

        assert!(Pattern::new(".b.".to_string()).unwrap().matches("abc".to_string()));
        assert!(Pattern::new(".bc.".to_string()).unwrap().matches("abcd".to_string()));
        assert!(Pattern::new(".b.d.".to_string()).unwrap().matches("abcde".to_string()));
        assert!(Pattern::new(".b..d.".to_string()).unwrap().matches("abcxde".to_string()));
        assert!(!Pattern::new(".b.x.d.".to_string()).unwrap().matches("abcxde".to_string()));

    }
}