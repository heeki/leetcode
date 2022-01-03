use palindrome_number::Solution;

fn main() {
    let input = 121;
    let answer = Solution::is_palindrome(input);
    println!("input={}, answer={}", input, answer);
}
