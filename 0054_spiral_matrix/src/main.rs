use spiral_matrix::Solution;

fn main() {
    let input: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::spiral_order(input);
    let input: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    Solution::spiral_order(input);
}