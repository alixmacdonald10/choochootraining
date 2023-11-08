// // Given an integer x, return true if x is a palindrome, and false otherwise.

// // Example 1:

// // Input: x = 121
// // Output: true
// // Explanation: 121 reads as 121 from left to right and from right to left.

// // Example 2:

// // Input: x = -121
// // Output: false
// // Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

// // Example 3:

// // Input: x = 10
// // Output: false
// // Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// // Constraints:

// //     -231 <= x <= 231 - 1

// // Follow up: Could you solve it without converting the integer to a string?

// // use std::collections::VecDeque;

use crate::utils::solution_runner::Solution;


pub fn run() {
    
    let inputs = vec![121, -121, 10];
    let expected_outputs = vec![true, false, false];

    {
        let solution_stringy = Solution::new(
            inputs.clone(), 
            expected_outputs.clone(), 
            is_palindrome_stringy);

            solution_stringy.run();
    }

    {
        let solution_numeric = Solution::new(
            inputs.clone(), 
            expected_outputs.clone(), 
            is_palindrome_numeric);

            solution_numeric.run();
    }
}

fn is_palindrome_stringy(x: i32) -> bool {
    x.to_string().chars().collect::<String>() == x.to_string().chars().rev().collect::<String>()
}

fn is_palindrome_numeric(x: i32) -> bool {

    if x < 0 || (x != 0 && x % 10 == 0) {
        return false;
    }

    let mut half = 0;
    let mut x_mut = x;
    while x_mut > half {
        half = (half * 10) + (x_mut % 10);
        x_mut = x_mut / 10;
    }

    x_mut == half || x_mut == half / 10
    
}