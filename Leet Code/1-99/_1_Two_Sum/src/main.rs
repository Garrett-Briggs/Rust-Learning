//1. Two Sum

//Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//You may assume that each input would have exactly one solution, and you may not use the same element twice.
//You can return the answer in any order.

//Example 1:
//Input: nums = [2,7,11,15], target = 9
//Output: [0,1]
//Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//Example 2:

//Input: nums = [3,2,4], target = 6
//Output: [1,2]
//Example 3:

//Input: nums = [3,3], target = 6
//Output: [0,1]

//Constraints:
//2 <= nums.length <= 104
//-109 <= nums[i] <= 109
//-109 <= target <= 109
//Only one valid answer exists.

//Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?


pub struct Solution {
}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut the_hash = std::collections::HashMap::new(); // used to map/remember the numbers we have already past key=value of number, value=index for the number

        for (i, &num) in nums.iter().enumerate() {  // loops through an array, .iter().enumerate() hands us the current index and the value
            let complement = target - num; // we are looking for this complement ex.9 - 2 = {7}

            if let Some(&complement_index) = the_hash.get(&complement) { // check the hash if we already found this complement, if pair found return the complement_index and current index
                return vec![complement_index as i32, i as i32];
            }
            the_hash.insert(num, i); // add number and its index
        }
        vec![] // just for safety
    }
}
// Time Complexity: O(N)
// Loop through the array once, checking for the number in the hash map is O(1) time.
// Space Complexity: O(N)
// Worst case is finding the matching pair at the end of the array. We will have stored all the
// elements by then.

fn main() {
    let the_nums = [
        vec![2, 7, 11, 15],
        vec![3, 2, 4],
        vec![3, 3],
    ];
    let the_targets = [
        9,
        6,
        6
    ];
    let the_ans = [
        vec![0, 1],
        vec![1, 2],
        vec![0, 1],
    ];

        for (i, nums) in the_nums.iter().enumerate() {
            let result = Solution::two_sum(nums.clone(), the_targets[i]);
            assert_eq!(result, the_ans[i], "Test case {} failed! Expected {:?}, got {:?}", i, the_ans[i], result);
        }
}
