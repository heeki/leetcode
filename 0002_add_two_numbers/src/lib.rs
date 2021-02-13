#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

// Reference and credit for linked list implementation:
// https://rust-unofficial.github.io/too-many-lists/second.html
use std::cmp;

#[derive(Debug)]
pub struct List {
    pub head: Option<Box<ListNode>>
}
#[allow(dead_code)]
impl List {
    fn new(node: Option<Box<ListNode>>) -> Self {
        List { head: node }
    }

    fn push(&mut self, val: i32) {
        let node = Box::new(ListNode {
            val,
            next: self.head.take()
        });
        self.head = Some(node)
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
}

pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let vec1 = Solution::create_vec_from_node(l1);
        let vec2 = Solution::create_vec_from_node(l2);
        // println!("vec1={:?}", vec1);
        // println!("vec2={:?}", vec2);

        let mut result = vec![];
        let mut carry = 0;
        let ops1 = cmp::min(vec1.len(), vec2.len());
        let ops2 = cmp::max(vec1.len(), vec2.len());
        // println!("ops1={} ops2={}", ops1, ops2);

        for i in 0..ops1 {
            let mut sum = vec1[i] + vec2[i] + carry;
            carry = 0;
            if sum > 9 {
                sum -= 10;
                carry += 1;
            }
            // println!("ops1: result={} vec1={} vec2={} carry={}", sum, vec1[i], vec2[i], carry);
            result.push(sum);
        }

        for j in ops1..ops2 {
            let mut sum = 0;
            if vec1.len() == ops2 {
                sum = vec1[j] + carry;
            } else if vec2.len() == ops2 {
                sum = vec2[j] + carry;
            }
            carry = 0;
            if sum > 9 {
                sum -= 10;
                carry += 1;
            }
            // println!("ops2: result={}", sum);
            result.push(sum);
        }

        if carry > 0 {
            // println!("ops3: result={}", carry);
            result.push(carry);
        }

        // println!("result={:?}", result);
        let list = Solution::create_list_from_vec(result);
        list.head
    }

    // helper functions for computation
    pub fn create_vec_from_node(input: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        let mut current = input;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    pub fn create_list_from_vec(mut input: Vec<i32>) -> List {
        let mut list: List = List::new(None);
        input.reverse();
        for i in input {
            list.push(i);
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input1 = vec![2,4,3];
        let input2 = vec![5,6,4];
        let list1 = Solution::create_list_from_vec(input1);
        let list2 = Solution::create_list_from_vec(input2);
        let result = Solution::add_two_numbers(list1.head, list2.head);
        assert_eq!(Solution::create_vec_from_node(result), vec![7,0,8])
    }

    #[test]
    fn example_2() {
        let input1 = vec![0];
        let input2 = vec![0];
        let list1 = Solution::create_list_from_vec(input1);
        let list2 = Solution::create_list_from_vec(input2);
        let result = Solution::add_two_numbers(list1.head, list2.head);
        assert_eq!(Solution::create_vec_from_node(result), vec![0])
    }

    #[test]
    fn example_3() {
        let input1 = vec![9,9,9,9,9,9,9];
        let input2 = vec![9,9,9,9];
        let list1 = Solution::create_list_from_vec(input1);
        let list2 = Solution::create_list_from_vec(input2);
        let result = Solution::add_two_numbers(list1.head, list2.head);
        assert_eq!(Solution::create_vec_from_node(result), vec![8,9,9,9,0,0,0,1])
    }

    #[test]
    fn example_4() {
        let input1 = vec![2,4,9];
        let input2 = vec![5,6,4,9];
        let list1 = Solution::create_list_from_vec(input1);
        let list2 = Solution::create_list_from_vec(input2);
        let result = Solution::add_two_numbers(list1.head, list2.head);
        assert_eq!(Solution::create_vec_from_node(result), vec![7,0,4,0,1])
    }
}