use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut validation = VecDeque::new();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => validation.push_front(c),
                ')' => match validation.front() {
                    Some(x) => if *x == '(' { validation.pop_front(); () } else { validation.push_front(c) },
                    None => validation.push_front(c)
                },
                '}' => match validation.front() {
                    Some(x) => if *x == '{' { validation.pop_front(); () } else { validation.push_front(c) },
                    None => validation.push_front(c)
                },
                ']' => match validation.front() {
                    Some(x) => if *x == '[' { validation.pop_front(); () } else { validation.push_front(c) },
                    None => validation.push_front(c)
                },
                _ => ()
            };
        }
        validation.len() == 0
    }

    pub fn is_valid_hashmap(s: String) -> bool {
        let mut validation = HashMap::new();
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => *validation.entry(c).and_modify(|i| *i += 1).or_insert(1),
                ')' => *validation.entry('(').and_modify(|i| *i -= 1).or_insert(-1),
                '}' => *validation.entry('{').and_modify(|i| *i -= 1).or_insert(-1),
                ']' => *validation.entry('[').and_modify(|i| *i -= 1).or_insert(-1),
                _ => 0
            };
        }
        let mut response = true;
        for (_, v) in validation.into_iter() {
            if v != 0 {
                response = false;
            }
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = String::from("()");
        let answer = Solution::is_valid(input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_2() {
        let input = String::from("()[]{}");
        let answer = Solution::is_valid(input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_3() {
        let input = String::from("(]");
        let answer = Solution::is_valid(input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_4() {
        let input = String::from("([])");
        let answer = Solution::is_valid(input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_5() {
        let input = String::from("([)]");
        let answer = Solution::is_valid(input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_6() {
        let input = String::from("]");
        let answer = Solution::is_valid(input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_7() {
        let input = String::from("(])");
        let answer = Solution::is_valid(input);
        assert_eq!(answer, false);
    }
}