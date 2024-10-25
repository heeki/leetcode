use valid_parenthesis::Solution;
use serde_json::json;

fn main() {
    let input = String::from("()");
    let answer = Solution::is_valid(input);
    let response = json!({
        "answer": answer
    });
    println!("{}", response);
}
