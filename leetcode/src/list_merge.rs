// https://leetcode.com/problems/median-of-two-sorted-arrays/

fn try_push<T: Ord + Clone>(vec: &mut Vec<T>, t: &T) {
    match vec.last() {
        None => vec.push(t.clone()),
        Some(last) => if *t > *last {vec.push(t.clone())},
    }
}

fn merge<T: Ord + Clone>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {

    let mut merged: Vec<T> = vec![];
    let mut iter1 = vec1.into_iter();
    let mut iter2 = vec2.into_iter();
    let mut next1_opt= iter1.next();
    let mut next2_opt= iter2.next();

    while next1_opt.is_some() || next2_opt.is_some() {
        match (&next1_opt, &next2_opt) {
            (Some(t1), Some(t2)) => {
                if *t1 < *t2 {
                    try_push(&mut merged, t1);
                    next1_opt = iter1.next();
                } else {
                    try_push(&mut merged, t2);
                    next2_opt = iter2.next();
                }
            }
            (Some(t1), None) => {
                try_push(&mut merged, t1);
                next1_opt = iter1.next();
            }
            (None, Some(t2)) => {
                try_push(&mut merged, t2);
                next2_opt = iter2.next();
            }
            (None, None) => {}
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use crate::list_merge::merge;
    #[test]
    fn test() {
        let l1 = vec!['a', 'c', 'e', 'g', 'h', 'i'];
        let l2 = vec!['b', 'd', 'f', 'h', 'i'];
        println!("{:?}", merge(l1, l2));
    }
}