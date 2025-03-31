// https://leetcode.com/problems/longest-substring-without-repeating-characters/
fn lss2(string: &str) -> &str {

    if string.len() < 2 {
        return string;
    }

    // Current left and right indices.
    let (mut i, mut j) = (0usize, 1usize);

    // Best result so far
    let mut result = &string[0..1];

    // Used to check that a new char is not already in the slice.
    let mut slice_set = std::collections::HashSet::new();
    slice_set.insert(&string[0..1]);

    while j < string.len() {
        let right = &string[j..j+1];
        if ! slice_set.insert(right) {
            // current right_char was a dupe.
            let candidate = &string[i..j];
            if candidate.len() > result.len() {
                result = candidate;
            }
            // advance i until it points one past the previous occurrence of `right`.
            while &string[i..i+1] != right {
                slice_set.remove(&string[i..i+1]);
                i += 1;
            }
            i += 1;
        }
        j += 1;
    }

    // Check the tail section
    let candidate = &string[i..j];
    if candidate.len() > result.len() {
        result = candidate;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::longest_substring::lss2;
    use rand::Rng;

    #[test]
    fn test() {
        fn next_string() -> String {
            let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
            let mut rng = rand::rng();
            (0..10).map(|_| chars[rng.random_range(0..chars.len())]).collect::<String>()
        }
        let string = next_string();
        println!("{} => {:?}", string, lss2(&string));
    }
}