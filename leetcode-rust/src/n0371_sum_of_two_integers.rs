/**
 * [371] Sum of Two Integers
 *
 * Calculate the sum of two integers a and b, but you are not allowed to use the operator + and -.
 * 
 * <div>
 * Example 1:
 * 
 * 
 * Input: a = <span id="example-input-1-1">1</span>, b = <span id="example-input-1-2">2</span>
 * Output: <span id="example-output-1">3</span>
 * 
 * 
 * <div>
 * Example 2:
 * 
 * 
 * Input: a = -<span id="example-input-2-1">2</span>, b = <span id="example-input-2-2">3</span>
 * Output: 1
 * 
 * </div>
 * </div>
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let r = a ^ b;
        let t = (a & b) << 1;

        if t == 0 {
            return r
        }

        Self::get_sum(t, r)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_371() {
        assert_eq!(Solution::get_sum(1, 2), 3);
        assert_eq!(Solution::get_sum(-2, 3), 1);
        assert_eq!(Solution::get_sum(-2, -3), -5);
        assert_eq!(Solution::get_sum(3, -1), 2);
    }
}
