/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fn matrix_multiply(a: &[[i64; 2]; 2], b: &[[i64; 2]; 2]) -> [[i64; 2]; 2] {
        let mut result = [[0, 0], [0, 0]];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }

    fn matrix_pow(mut matrix: [[i64; 2]; 2], mut n: i64) -> [[i64; 2]; 2] {
        let mut result = [[1, 0], [0, 1]];
        while n > 0 {
            if n % 2 == 1 {
                result = matrix_multiply(&result, &matrix);
            }
            matrix = matrix_multiply(&matrix, &matrix);
            n /= 2;
        }
        result
    }

    let base_matrix = [[1, 1], [1, 0]];
    let result_matrix = matrix_pow(base_matrix, (n - 1) as i64);
    result_matrix[0][0] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
