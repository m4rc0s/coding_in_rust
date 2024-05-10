fn main() {
    let n = 11;

    let mut sequence_numbers_storage = Vec::new();
    let res = collatz_sequence(n, &mut sequence_numbers_storage);

    println!(
        "Collatz sequence is {:?} and the size is {:?}",
        res,
        res.len()
    );
}

pub fn collatz_sequence(n: i32, result: &mut Vec<i32>) -> &Vec<i32> {
    result.push(n);
    if n == 1 {
        return result;
    }

    if n % 2 == 0 {
        return collatz_sequence(n / 2, result);
    } else {
        return collatz_sequence(3 * n + 1, result);
    }
}


#[cfg(test)]
mod tests {
    use super::collatz_sequence;

    #[test]
    fn should_build_collatz_sequence_for_given_n() {
        let expected_value_for_n = 11;
        let expected_collatz_length = 15;
        let mut expected_sequence_numbers_storage = Vec::new();

        let actual = collatz_sequence(expected_value_for_n, &mut expected_sequence_numbers_storage);

        assert_eq!(actual.len(), expected_collatz_length)
    }
}
