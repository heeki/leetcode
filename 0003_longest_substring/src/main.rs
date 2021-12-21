use longest_substring::Solution;
use serde_json::json;

fn main() {
    let input = String::from("pwwkew");
    let answer = Solution::length_of_longest_substring(input);
    let response = json!({
        "answer": answer
    });
    println!("{}", response);
}