/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.
 * 
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * 
 * Example:
 * 
 * 
 * Given nums = [2, 7, 11, 15], target = 9,
 * 
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 * 
 * 
 *  
 * 
 */
pub struct Solution {}

// submission codes start here
use std::collections::HashMap;
impl Solution {
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut result = vec![];

    //     for (k,i) in nums.iter().enumerate() {
    //         for (k2, i2) in nums.iter().enumerate() {
    //             if k2 > k && i+i2 == target {
    //                 result.push(k as i32);
    //                 result.push(k2 as i32);
    //             }
    //         }
    //     }

    //     result
    // }

    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let len = nums.len();

    //     for i in 0..len {
    //         for j in i+1..len {
    //             if nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }

    //     vec![]
    // }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hmap = HashMap::new();

        for i in 0..nums.len() {
            match hmap.get(&(target-nums[i] as i32)) {
                None => {hmap.insert(nums[i], i);},
                Some(v) => {
                    return vec![*v as i32, i as i32];
                },
            }
        }

        //dbg!(hmap);

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![99, 2, 11, 15], 114), vec![0, 3]);
    }
}
