fn main() {}

pub fn create_array_and_modify() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;

    println!("the array is {a:?}");
}

pub fn tuples() {
    let t: (i8, bool) = (7, true);

    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}

pub fn interact_with_array() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        println!("the analyzed prime number is {:?}", prime);
        for i in 2..prime {
            println!("divisor is: {:?}", i);
            println!("division res: {:?}", prime % i);
            assert_ne!(prime % i, 0);
        }
    }
}

pub fn pattern_destructuring() {
    let tup: (i32, i32) = (13, 15);

    let (left, right) = tup;

    println!("left: {left}, right: {right}");
}

pub fn transpose_matrix(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut _resultant_matrix: [[i32; 3]; 3] = [[1; 3]; 3];

    for i in 0..matrix.len() {
        println!("this is the line: {:?}", matrix[i]);
        for j in 0..matrix[i].len() {
            println!("this is the line item {:?}", matrix[i][j]);
            _resultant_matrix[j][i] = matrix[i][j];
        }
    }

    return _resultant_matrix;
}

#[cfg(test)]
mod tests {
    use crate::transpose_matrix;

    #[test]
    fn should_transpound_matrix() {
        let original_matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let expected_transposed_matrix = [[1, 4, 7], [2, 5, 8], [3, 6, 9]];

        let actual = transpose_matrix(original_matrix);

        assert_eq!(actual, expected_transposed_matrix);
    }
}
