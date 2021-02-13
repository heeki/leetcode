use add_two_numbers::Solution;

fn main() {
    let input1 = vec![2,4,3];
    let input2 = vec![5,6,4];
    let list1 = Solution::create_list_from_vec(input1);
    let list2 = Solution::create_list_from_vec(input2);
    println!("list1={:?}", list1);
    println!("list2={:?}", list2);
    let validation = Solution::add_two_numbers(list1.head, list2.head);
    println!("validation={:?}", validation);

    let input1 = vec![9,9,9,9,9,9,9];
    let input2 = vec![9,9,9,9];
    let list1 = Solution::create_list_from_vec(input1);
    let list2 = Solution::create_list_from_vec(input2);
    println!("list1={:?}", list1);
    println!("list2={:?}", list2);
    let validation = Solution::add_two_numbers(list1.head, list2.head);
    println!("validation={:?}", validation);

    let input1 = vec![2,4,9];
    let input2 = vec![5,6,4,9];
    let list1 = Solution::create_list_from_vec(input1);
    let list2 = Solution::create_list_from_vec(input2);
    println!("list1={:?}", list1);
    println!("list2={:?}", list2);
    let validation = Solution::add_two_numbers(list1.head, list2.head);
    println!("validation={:?}", validation);
}
