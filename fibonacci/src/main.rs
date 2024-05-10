fn main() {
    for n in 0..29 {
        if n > 2 {
            let n_minus_2 = n - 2;
            let n_minus_1 = n - 1;
            let fib_a = fib(n_minus_2);
            let fib_b = fib(n_minus_1);

            println!(
                "fib({n}) = fib({:?}) + fib({:?}) = {:?}",
                n_minus_2,
                n_minus_1,
                fib_a + fib_b
            )
        } else {
            println!(
                "Fibonacci of {n} is {:?} + {:?} = {:?}",
                fib(n),
                fib(n),
                fib(n)
            )
        }
    }
}

pub fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 2) + fib(n - 1),
    }
}

pub fn fib_a(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn should_calculate_fibonacci_for_a_number() {
        let expected_value_for_n = 10;
        let expected_result = 55;

        let actual = fib(expected_value_for_n);

        assert_eq!(actual, expected_result)
    }
}
