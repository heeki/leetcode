use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        Solution::optimized_implementation(s)
    }

    // looked up the provided python solution
    fn optimized_implementation(s: String) -> i32 {
        let mut counts = HashMap::new();
        let mut response = 0;
        let mut pos_l = 0;
        let mut pos_r = 0;
        while pos_r < s.len() {
            let char_r = s.chars().nth(pos_r).unwrap();
            let mut entry = counts.entry(char_r).or_insert(0);
            *entry += 1;
            while *entry > 1 {
                let char_l = s.chars().nth(pos_l).unwrap();
                entry = counts.entry(char_l).or_insert(0);
                *entry -= 1;
                pos_l += 1;
                entry = counts.entry(char_r).or_insert(0);
            }
            // println!("response={}, pos_r={}, pos_l={}", response, pos_r, pos_l);
            response = cmp::max(response, pos_r - pos_l + 1);
            pos_r += 1;
        }
        i32::try_from(response).unwrap()
    }

    #[allow(dead_code)]
    fn naive_implementation(s: String) -> i32 {
        let mut response = 0;
        let mut has_repeats = true;
        for length in (1..s.len()+1).rev() {
            println!("length={}", length);
            for start in 0..s.len()-length+1 {
                let substring = &s[start..start+length];
                has_repeats = Solution::has_repeats(substring);
                if length > response && !has_repeats {
                    response = length;
                    // println!("i={}, substring={}, has_repeats={}, response={}", i, substring, has_repeats, response);
                    break;
                }
            }
            if !has_repeats {
                break;
            }
        }
        i32::try_from(response).unwrap()
    }

    #[allow(dead_code)]
    fn has_repeats(s: &str) -> bool {
        let mut counts = HashMap::new();
        for c in s.chars() {
            let counter = counts.entry(c).or_insert(0);
            *counter += 1;
        }
        let mut response = false;
        for (_k, v) in counts.iter() {
            if *v > 1 {
                response = true;
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
        let input = String::from("abcabcbb");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_2() {
        let input = String::from("bbbbb");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_3() {
        let input = String::from("pwwkew");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 3);
    }

    #[test]
    fn test_4() {
        let input = String::from("");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_5() {
        let input = String::from(" ");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_6() {
        let input = String::from("au");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_7() {
        let input = String::from("aa");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_8() {
        let input = String::from("aab");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_9() {
        let input = String::from("bwf");
        let answer = Solution::length_of_longest_substring(input);
        assert_eq!(answer, 3);
    }
}