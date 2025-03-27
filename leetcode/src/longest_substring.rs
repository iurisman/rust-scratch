// https://leetcode.com/problems/add-two-numbers/
fn add_two_numbers(l1: &Vec<u8>, l2: &Vec<u8>) -> Vec<u8> {
    let mut i1 = l1.iter().rev();
    let mut i2 = l2.iter().rev();
    let mut carry = 0;
    let mut result = Vec::<u8>::new();
    loop {
        let sum_opt = match (i1.next(), i2.next()) {
            (None, None) => None,
            (Some(x), Some(y)) => Some(carry + x + y),
            (None, Some(y)) => Some(y + carry),
            (Some(x), None) => Some(x + carry),
        };
        match sum_opt {
            None => break,
            Some(sum) => {
                carry = sum / 10;
                result.insert(0, sum - carry * 10);
            }
        }
    }
    if carry > 0 {
        result.insert(0, carry);
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers::add_two_numbers;

    #[test]
    fn test_add_two_numbers() {
        println!("{:?}", add_two_numbers(&vec![2, 4, 3], &vec![5, 6, 4]));
    }
}