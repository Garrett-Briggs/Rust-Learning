pub struct Solution { }

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(2 * nums.len());

        let index = n as usize;

        for i in 0..index {
            ans.push(nums[i]);
            ans.push(nums[i + index]);
        }
        ans
    }

}
// Time Complexity: O(N)
// Loop through the array once, adding each index to ans one time is O(N)
// Space Complexity: O(N)
// The nums array is equal to the ans array so O(N)
// elements by then.

fn main() {
    let the_nums = [
        vec![2,5,1,3,4,7],
        vec![1,2,3,4,4,3,2,1],
        vec![1, 1, 2, 2],
    ];
    let the_ns = [
        3, 4, 2,
    ];
    let the_ans = [
        vec![2,3,5,4,1,7],
        vec![1,4,2,3,3,2,4,1],
        vec![1,2,1,2],
    ];

        for (i, nums) in the_nums.iter().enumerate() {
            let result = Solution::shuffle(nums.clone(), the_ns[i]);
            assert_eq!(result, the_ans[i], "Test case {} failed! Expected {:?}, got {:?}", i, the_ans[i], result);
        }
        println!("All tests passed")
}
