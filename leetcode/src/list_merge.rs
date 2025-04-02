// https://leetcode.com/problems/median-of-two-sorted-arrays/

fn merge<T: Ord + Clone>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {

    let mut merged: Vec<T> = vec![];

    let mut iter1 = vec1.into_iter();
    let mut iter2 = vec2.into_iter();
    let mut next1_opt= iter1.next();
    let mut next2_opt= iter2.next();

    while next1_opt.is_some() || next2_opt.is_some() {
        match (&next1_opt, &next2_opt) {
            (Some(t1), Some(t2)) => {
                if *t1 > *t2 {
                    merged.push((*t1).clone());
                    next1_opt = iter1.next();
                } else {
                    merged.push((*t2).clone());
                    next2_opt = iter2.next();
                }
            }
            (Some(t1), None) => {
                merged.push((*t1).clone());
                next1_opt = iter1.next();
            }
            (None, Some(t2)) => {
                merged.push((*t2).clone());
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
        let l1 = vec!['a', 'c', 'e', 'g'];
        let l2 = vec!['b', 'd', 'f', 'h', 'i'];
        println!("{:?}", merge(l1, l2));
    }
}