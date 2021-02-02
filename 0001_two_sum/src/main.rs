use two_sum::Solution;

fn main() {
    let input: Vec<i32> = vec![2,7,11,15];
    let target: i32 = 9;
    Solution::two_sum(input, target);
    let input: Vec<i32> = vec![3,2,4];
    let target: i32 = 6;
    Solution::two_sum(input, target);
    let input: Vec<i32> = vec![3,3];
    let target: i32 = 6;
    Solution::two_sum(input, target);
}
