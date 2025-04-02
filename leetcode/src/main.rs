
mod add_two_numbers;
mod longest_substring;
mod list_merge;
fn main() {
    let l1 = vec!['a', 'c', 'e', 'g'];
    let l2 = vec!['b', 'd', 'f', 'h', 'i'];
    println!("{:?}", merge(l1, l2));
}

// https://leetcode.com/problems/median-of-two-sorted-arrays/

fn merge<T: Ord>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {

    let mut merged: Vec<T> = vec![];

    let mut iter1 = vec1.into_iter();
    let mut next1_opt= iter1.next();

    while next1_opt.is_some() {
        match next1_opt {
            Some(t) => {
                if t > t {
                    merged.push(t);
                    next1_opt = iter1.next();
                }
                else {
                    next1_opt = None;
                }
            }
            None => {
                next1_opt = iter1.next();
            }
        }


    }
    merged
}
