//use std::collections::HashMap;

pub struct Solution { }

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        //let mut the_hash: HashMap<i32, i32> = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if i as i32 + 1_i32 != *v {
                return vec![*v, i as i32 + 1_i32];
            }
        }

        //for (i, v) in nums.iter().enumerate() {
        //    if let Some(&_index) = the_hash.get(v) {
        //        return vec![*v, i as i32 + 1_i32]
        //    }
        //    the_hash.insert(*v, i as i32);
        //}
       
        //for v in nums.iter() {
        //    if let Some(&index) = the_hash.get(v) {
        //        return vec![*v, index + 1]
        //    }
        //    the_hash.insert(*v, *v);
        //}
        vec![]
    }
}
// Time Complexity: O(N)
// Iterates through the array once ⇒ O(N)
// Space Complexity: O(1)
// Worst the whole vector is converted to hash ⇒ O(N)
// elements by then.

fn main() {
    let the_nums = [
        vec![1,2,2,4],
        vec![1,1],
        vec![1,2,3,2],
    ];
    let the_ans = [
        vec![2,3],
        vec![1,2],
        vec![2,4],
    ];

        for (i, nums) in the_nums.iter().enumerate() {
            let result = Solution::find_error_nums(nums.clone());
            assert_eq!(result, the_ans[i], "Test case {} failed! Expected {:?}, got {:?}", i, the_ans[i], result);
        }
        println!("All tests passed")
}
