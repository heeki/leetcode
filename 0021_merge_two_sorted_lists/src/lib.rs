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
        let mut vec1 = Solution::create_vec_from_node(list1);
        let mut vec2 = Solution::create_vec_from_node(list2);
        let mut merged = vec![];

        while vec1.len() != 0 || vec2.len() != 0 {
            let first1 = vec1.first();
            let first2 = vec2.first();
            match (first1, first2) {
                (Some(f1), Some(f2)) => {
                    // println!("{:?} {:?} -> {} {}", vec1, vec2, *f1, *f2);
                    if f1 <= f2 {
                        merged.push(*f1);
                        vec1.remove(0);
                    } else {
                        merged.push(*f2);
                        vec2.remove(0);
                    }
                },
                (Some(f1), None) => {
                    // println!("{:?} {:?} -> {}", vec1, vec2, *f1);
                    merged.push(*f1);
                    vec1.remove(0);
                },
                (None, Some(f2)) => {
                    // println!("{:?} {:?} -> {}", vec1, vec2, *f2);
                    merged.push(*f2);
                    vec2.remove(0);
                },
                (None, None) => ()
            }
            // println!("  -> {:?}", merged);
        }
        Solution::create_list_from_vec(merged).head
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

    #[test]
    fn test_2() {
        let input1 = vec![];
        let input2 = vec![];
        let output = vec![];
        let list1 = Solution::create_list_from_vec(input1);
        let list2 = Solution::create_list_from_vec(input2);
        let answer = Solution::merge_two_lists(list1.head, list2.head);
        assert_eq!(Solution::create_vec_from_node(answer), output);
    }

    #[test]
    fn test_3() {
        let input1 = vec![];
        let input2 = vec![0];
        let output = vec![0];
        let list1 = Solution::create_list_from_vec(input1);
        let list2 = Solution::create_list_from_vec(input2);
        let answer = Solution::merge_two_lists(list1.head, list2.head);
        assert_eq!(Solution::create_vec_from_node(answer), output);
    }
}