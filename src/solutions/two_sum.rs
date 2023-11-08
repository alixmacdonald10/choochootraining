// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

 
// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]


// Constraints:

//     2 <= nums.length <= 104
//     -109 <= nums[i] <= 109
//     -109 <= target <= 109
//     Only one valid answer exists.

// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
use std::collections::HashMap;

use crate::utils::solution_runner::SolutionWithTarget;

pub fn run() {

    let input_vec = vec![vec![2, 7, 11, 15], vec![3, 2, 4], vec![3, 3]];
    let target_vec = vec![9, 6, 6];
    let expected_vec = vec![vec![0, 1], vec![1, 2], vec![0, 1]];

    {
        println!("Running with n2 complexity...");
        let solution_squared_time = SolutionWithTarget::new(
            input_vec.clone(), 
            target_vec.clone(), 
            expected_vec.clone(), 
            two_sum_squared_time);

            solution_squared_time.run(true);
    }

    {
        println!("Running with n complexity...");
        let solution_constant_time = SolutionWithTarget::new(
            input_vec, 
            target_vec, 
            expected_vec, 
            two_sum_constant_time);

        solution_constant_time.run(true);
    }
}


/// O(n^2) solution
pub fn two_sum_squared_time(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut result = Vec::new();
    'outer: for (i_idx, i) in nums.clone().into_iter().enumerate() {
        for (j_idx, j) in nums.clone().into_iter().enumerate() {
            if i_idx == j_idx {
                continue;
            } else {
                if i + j == target {
                    result.push(i_idx as i32);
                    result.push(j_idx as i32);
                    break 'outer
                }
            }
        }
    }
    result
}

/// O(n) solution
pub fn two_sum_constant_time(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut hashmap = HashMap::new();
    let mut result: Vec<i32> = Vec::with_capacity(2);

    for i in 0..nums.len() {
        hashmap.insert(nums[i], i);
    }

    for i in 0..nums.len() {
        let complement = target - nums[i];
        if hashmap.contains_key(&complement) && hashmap.get(&complement).unwrap() != &i {
            result = vec![i as i32, *hashmap.get(&complement).unwrap() as i32];
            break
        }
    }
    
    result
}