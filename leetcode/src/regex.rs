use crate::push_back_itrerator::PushBackChars;

// https://leetcode.com/problems/regular-expression-matching/
// Supports *, ., \ escape

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
        let mut escaping = false;
        let mut input_chars = PushBackChars::new(word.chars());
        while let Some(char) = input_chars.next() {
            if pattern_ix == self.pattern_chars.len() {
                // We've exhausted the pattern, but not yet the string.
                // the last pattern char was not '*'
                return false;
            }
            match self.pattern_chars[pattern_ix] {
                '\\' if !escaping => {
                    escaping = true;
                    pattern_ix = pattern_ix + 1;
                    // Re-evaluate the current char as escaped.
                    input_chars.push(char);
                },
                '.' if !escaping => {
                    pattern_ix = pattern_ix + 1;
                }
                '*' => if !escaping {
                    // Don't get stuck here if the current input char matches the next pattern char
                    if pattern_ix == self.pattern_chars.len() - 1 {
                        // Short-circuit because the * is the last char in pattern,
                        // so we don't care what's left of the string.
                        return true;
                    } else if char == self.pattern_chars[pattern_ix + 1] {
                        pattern_ix = pattern_ix + 2;
                    }
                }
                c if c == char => {
                    pattern_ix = pattern_ix + 1;
                    escaping = false;
                }
                _ => return false
            }
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

        assert!(Pattern::new(".b*d.".to_string()).unwrap().matches("abcxde".to_string()));
        assert!(Pattern::new(".b*d*".to_string()).unwrap().matches("abcxde".to_string()));
        assert!(Pattern::new("*b*d.".to_string()).unwrap().matches("abcxde".to_string()));

        assert!(Pattern::new("ab.".to_string()).unwrap().matches("ab.".to_string()));
        assert!(Pattern::new("ab.".to_string()).unwrap().matches("ab*".to_string()));
        assert!(Pattern::new("ab*".to_string()).unwrap().matches("ab*".to_string()));
        assert!(Pattern::new(r#"ab\."#.to_string()).unwrap().matches("ab.".to_string()));
        assert!(!Pattern::new(r#"ab\."#.to_string()).unwrap().matches("ab.d".to_string()));
        assert!(!Pattern::new(r#"ab\."#.to_string()).unwrap().matches("abcd".to_string()));
        assert!(!Pattern::new(r#"ab\.d"#.to_string()).unwrap().matches("abcd".to_string()));
        assert!(Pattern::new(r#"ab\.d"#.to_string()).unwrap().matches("ab.d".to_string()));
    }
}