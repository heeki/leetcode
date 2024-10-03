use roman_to_integer::Solution;
use serde_json::json;

fn main() {
    let input = String::from("III");
    let answer = Solution::roman_to_int(input);
    let response = json!({
        "answer": answer
    });
    println!("{}", response);
}
