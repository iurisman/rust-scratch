
mod add_two_numbers;
mod longest_substring;
mod list_merge;
fn main() {
}

// https://leetcode.com/problems/median-of-two-sorted-arrays/

fn f1<T: Ord>(vec1: Vec<T>) -> Vec<T> {

    let mut merged: Vec<T> = vec![];

    let mut iter1 = vec1.into_iter();
    let mut next1_opt= iter1.next();

    // while next1_opt.is_some() {
    //     match next1_opt {
    //         Some(t) => {
    //             if true {
    //                 merged.push(t);
    //                 next1_opt = iter1.next();
    //             }
    //         }
    //         None => {}
    //     }
    // }
    merged
}
