// https://leetcode.com/problems/longest-palindromic-substring/

/** Brute force search for longest palindromic substring */
fn search(string: &str) -> &str {
    let mut result = &string[0..1];
    for i in 1..string.len() - 1 {
        for j in (i + 1)..string.len() {
            let candidate = &string[i..j];
            if candidate.len() > result.len() && is_palindrome(candidate) {
                result = candidate;
            }
        }
    }
    result
}

fn is_palindrome(string: &str) -> bool {
    *string == string.chars().rev().collect::<String>()
}
#[cfg(test)]
mod tests {
    use crate::longest_palindromic_substring::search;
    use rand::Rng;

    #[test]
    fn test() {
        fn next_string() -> String {
            const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
            let mut rng = rand::rng();
            (0..200).map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char).collect()
        }

        let string = next_string();
        println!("{} => {:?}", string, search(&string));
    }
}