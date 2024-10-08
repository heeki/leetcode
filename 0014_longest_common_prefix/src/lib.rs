pub struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        for (i, s) in strs.iter().enumerate() {
            println!("strs[{}]={}", i, s);
            let slice = &strs[i+1..strs.len()];
            for (j, t) in slice.iter().enumerate() {
                println!("  -> strs[{}]={}", j, t);
            }
        }
        String::from("hello")
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
}