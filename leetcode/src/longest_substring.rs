// https://leetcode.com/problems/longest-substring-without-repeating-characters/
fn lss(string: &str) -> String {

    if string.len() < 2 {
        return String::from(string);
    }

    // Convert input to an array. This is an expensive copy that we'll have to deal with.
    let string_array = string.chars().collect::<Vec<char>>();
    let (mut i, mut j) = (0usize, 1usize);
    // Used to check that a new char is not already in the slice.
    let mut slice_set = std::collections::HashSet::new();
    slice_set.insert(string_array[i]);
    let mut result: Vec<char> = Vec::new();
//    let mut left_char_to_eliminate: Option<char> = None;
//    let mut already_eliminated = false;
    while j < string_array.len() {
        let right = string_array[j];
        if ! slice_set.insert(right) {
            // current right_char was a dupe.
            let candidate = &string_array[i..j];
            if candidate.len() > result.len() {
                result = candidate.to_vec();
            }
            // advance i until it points one past the previous occurrence of `right`.
            while string_array[i] != right {
                slice_set.remove(&string_array[i]);
                i += 1;
            }
            i += 1;
        }
        j += 1;
    }

    // Check the tail section
    let candidate = &string_array[i..j];
    if candidate.len() > result.len() {
        result = candidate.to_vec();
    }

    let foo: String = result.iter().copied().collect();
    foo
}



#[cfg(test)]
mod tests {
    use crate::longest_substring::lss;
    use rand::Rng;

    #[test]
    fn test() {
        fn next_string() -> String {
            let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
            let mut rng = rand::rng();
            (0..8).map(|_| chars[rng.random_range(0..chars.len())]).collect::<String>()
        }
        // let string = "hdfdhcbe"; //      => "hdf"
        // println!("{} => {:?}", string, lss(string));
        let string = next_string();
        println!("{} => {:?}", string, lss(&string));
    }
}