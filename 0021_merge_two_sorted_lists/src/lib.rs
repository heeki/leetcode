
// reference and credit for linked list implementation
// https://rust-unofficial.github.io/too-many-lists/second.html
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

// definition for singly-linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        println!("{:?}", list1);
        println!("{:?}", list2);

        list1
    }

    // helper functions
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
    fn test_1() {
        let input1 = vec![1,2,4];
        let input2 = vec![1,3,4];
        let output = vec![1,1,2,3,4,4];
        let list1 = Solution::create_list_from_vec(input1);
        let list2 = Solution::create_list_from_vec(input2);
        let answer = Solution::merge_two_lists(list1.head, list2.head);
        assert_eq!(Solution::create_vec_from_node(answer), output);
    }
}