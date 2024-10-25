use merge_two_sorted_lists::Solution;
use serde_json::json;

fn main() {
    let input1 = vec![1,2,4];
    let input2 = vec![1,3,4];
    let list1 = Solution::create_list_from_vec(input1);
    let list2 = Solution::create_list_from_vec(input2);
    let answer = Solution::merge_two_lists(list1.head, list2.head);
    let response = json!({
        "answer": Solution::create_vec_from_node(answer)
    });
    println!("{}", response);
}