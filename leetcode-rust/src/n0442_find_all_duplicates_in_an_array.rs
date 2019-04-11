/**
 * [442] Find All Duplicates in an Array
 *
 * Given an array of integers, 1 &le; a[i] &le; n (n = size of array), some elements appear twice and others appear once.
 * 
 * Find all the elements that appear twice in this array.
 * 
 * Could you do it without extra space and in O(n) runtime?
 * 
 * Example:<br/>
 * 
 * Input:
 * [4,3,2,7,8,2,3,1]
 * 
 * Output:
 * [2,3]
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut nums = nums;

        for i in 0..nums.len() {
            let abs = nums[i].abs() -1;
            //dbg!(abs);

            //1 ≤ a[i] ≤ n
            if (nums[abs as usize] > 0) {
               // contains()
                nums[abs as usize] *= -1;
            } else {
                result.push(abs+1);
            }
        }
        //dbg!(nums);
        //dbg!(result.clone());

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_442() {
        assert_eq!(Solution::find_duplicates(vec![4,3,2,7,8,2,3,1]), vec![2,3]);
        //1 ≤ a[i] ≤ n
        // 说明是整数
    }
}
