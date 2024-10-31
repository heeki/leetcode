use remove_duplicates::Solution;
use serde_json::json;

fn main() {
    let mut input: Vec<i32> = vec![1,1,2];
    let answer = Solution::remove_duplicates(&mut input);
    let response = json!({
        "answer": answer
    });
    println!("{}", response);
}
