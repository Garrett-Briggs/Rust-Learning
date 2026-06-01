use std::cmp;

pub struct Solution { }

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let mut max: i32 = 0;

        for &num in &nums {
            if num == 1 {
                count += 1;
            } else {
                max = cmp::max(max, count);
                count = 0;
            }
        }
        cmp::max(max, count)
    }
}
// Time Complexity: O(N)
// Loop through the array once, time is O(N)
// Space Complexity: O(1)
// Only count, max are created with usize so O(1)
// elements by then.

fn main() {
    let the_nums = [
        vec![1,1,0,1,1,1],
        vec![1,0,1,1,0,1],
    ];
    let the_ans = [
        3, 2,
    ];

        for (i, nums) in the_nums.iter().enumerate() {
            let result = Solution::find_max_consecutive_ones(nums.clone());
            assert_eq!(result, the_ans[i], "Test case {} failed! Expected {:?}, got {:?}", i, the_ans[i], result);
        }
        println!("All tests passed")
}
