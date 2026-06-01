pub struct Solution {
}
impl Solution {
        pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        // nums.repeat(2)

        let mut ans: Vec<i32> = Vec::with_capacity(2 * nums.len());
        ans.extend(nums.iter().copied());
        ans.extend(nums.iter().copied());
        ans
    }
}
// Time Complexity: O(N)
// Loop through the array once, adding nums each time is O(2N) ⇒ O(N)
// Space Complexity: O(N)
// The nums array must be copied 2 times so O(2N) ⇒ O(N)
// elements by then.

fn main() {
    let the_nums = [
        vec![1, 2, 1],
        vec![1, 3, 2, 1],
    ];
    let the_ans = [
        vec![1, 2, 1, 1, 2, 1],
        vec![1, 3, 2, 1, 1, 3, 2, 1],
    ];

        for (i, nums) in the_nums.iter().enumerate() {
            let result = Solution::get_concatenation(nums.clone());
            assert_eq!(result, the_ans[i], "Test case {} failed! Expected {:?}, got {:?}", i, the_ans[i], result);
        }
    println!("All tests pass");
}
