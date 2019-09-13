//! Problem link: https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

/// We need check if the complement exists in the array. If the complement
/// exists, we need to look up its index. What is the best way to maintain
/// a mapping of each element in the array to its index? A hash-map.
///
/// A hash-map is built exactly for this purpose, it supports fast look
/// up in near constant time. I say "near" because if a collision occurred,
/// a look up could degenerate to O(n) time. But look up in hash
/// table should be amortized O(1) time as long as the hash
/// function was chosen carefully.
///
/// We iterate over the `nums` and check if each element's complement
/// (`target` - `nums[i]`) exists in the table if so we get it's index from the
/// hash-map and return both,
/// otherwise we insert that number along with it's index in the map.
///
/// Result:
///
/// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
/// (0.5ms)
///
/// Memory Usage: 3 MB, less than 50.00% of Rust online submissions for
/// Two Sum. (any improvements?)
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // pre-allocate :)
        let mut map = HashMap::with_capacity(nums.len());
        let mut result = vec![0; 2];
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                result[0] = *map.get(&complement).unwrap();
                result[1] = i as i32;
                break; // there is no more work :)
            } else {
                map.insert(num, i as i32);
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
