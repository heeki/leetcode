pub struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest: String = String::from("");

        // find the min length of any string in the vector
        let all_lengths: Vec<usize> = strs.iter().map(|s| s.len()).collect();
        let min_length: usize = match all_lengths.iter().min() {
            Some(length) => *length,
            None => 0
        };
        // println!("min_length={}", min_length);

        // check if the i-th element of each string is the same, up to min length
        for i in 0..min_length {
            let current: Vec<char> = strs.iter().map(|s| s.chars().nth(i).unwrap()).collect();
            // check all elements are the same using a sliding window of 2
            if current.windows(2).all(|s| s[0] == s[1]) {
                longest.push(current[0]);
            } else {
                break
            }
        }
        String::from(longest)
    }

    // initial incorrect implementation that searched for the longest common string anywhere, not just the prefix
    #[allow(unused_variables)]
    pub fn longest_common_string(strs: Vec<String>) -> String {
        let mut longest: String = String::from("");
        for (i, s) in strs.iter().enumerate() {
            // println!("strs[{}]={}", i, s);
            let comparisons = &strs[i+1..strs.len()];
            for (j, t) in comparisons.iter().enumerate() {
                // println!("\t-> strs[{}]={}", j, t);
                let mut x: usize = 0;
                let mut y: usize = 0;
                let mut current: String = String::from("");
                while x < s.chars().count() && y < t.chars().count() {
                    if s.chars().nth(x).unwrap() == t.chars().nth(y).unwrap() {
                        // println!("\t\t-> {} == {}", s.chars().nth(x).unwrap(), t.chars().nth(y).unwrap());
                        current.push(s.chars().nth(x).unwrap());
                        if current.len() > longest.len() {
                            longest = current.clone();
                        }
                    }
                    x += 1;
                    y += 1;
                }

            }
        }
        String::from(longest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let answer = Solution::longest_common_prefix(input);
        assert_eq!(answer, "fl");
    }

    #[test]
    fn test_2() {
        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let answer = Solution::longest_common_prefix(input);
        assert_eq!(answer, "");
    }

    #[test]
    fn test_3() {
        let input = vec!["cir".to_string(), "car".to_string()];
        let answer = Solution::longest_common_prefix(input);
        assert_eq!(answer, "c");
    }
}