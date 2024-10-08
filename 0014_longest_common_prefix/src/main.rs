use serde_json::{json};
use longest_common_prefix::Solution;

fn main() {
    let input = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let output = json!(input);
    let answer = Solution::longest_common_prefix(input);
    println!("input={}, answer={}", output.to_string(), answer);
}
