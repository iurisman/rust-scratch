use std::collections::{LinkedList, VecDeque};

// https://leetcode.com/problems/longest-substring-without-repeating-characters/
fn code(string: &str, rows: usize) -> String {
    let vec = string.chars().filter(|&c| c != ' ').collect::<Vec<char>>();
    let period = 2 * rows - 2;
    let mut columns = LinkedList::<Vec<char>>::new();
    let mut curr_column = vec![' '; rows];
    curr_column[0] = vec[0];
    for i in 1..vec.len() {
        let ix = i % period;
        if ix == 0 {
            // start new full column
            columns.push_back(curr_column);
            curr_column = vec![' '; rows];
            curr_column[0] = vec[i];
        } else if ix < rows {
            // indexing into current vertical rows
            curr_column[ix] = vec[i];
        } else {
            // start new partial column
            columns.push_back(curr_column);
            curr_column = vec![' '; rows];
            curr_column[period - ix] = vec[i]
        }
    }
    columns.push_back(curr_column);
println!("{:?}", columns);
    let mut result = Vec::<String>::new();
    for i in 0..rows {
        result.push(
            columns.iter().map(|col| col[i]).filter(|&c| c != ' ').collect::<String>()
        );
    };

    result.join("")
}

#[cfg(test)]
mod tests {
    use crate::zigzag_coder::code;

    #[test]
    fn test() {
        let string = "hello world";
        println!("{}", code(string, 3));
    }
}