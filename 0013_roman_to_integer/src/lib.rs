use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    fn is_substraction_case(current: char, next: Option<char>) -> bool {
        match (current, next) {
            ('I', Some('V' | 'X')) => true,
            ('X', Some('L' | 'C')) => true,
            ('C', Some('D' | 'M')) => true,
            _ => false,
        }
    }

    pub fn roman_to_int(x: String) -> i32 {
        let map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000)
        ]);
        let mut result = 0;
        for (i, c) in x.chars().enumerate() {
            let value = map.get(&c).copied().unwrap_or(0);
            if Solution::is_substraction_case(c, x.chars().nth(i+1)) {
                result -= value;
            } else {
                result += value;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = String::from("III");
        let answer = Solution::roman_to_int(input);
        assert_eq!(answer, 3);
    }
    #[test]
    fn test_2() {
        let input = String::from("LVIII");
        let answer = Solution::roman_to_int(input);
        assert_eq!(answer, 58);
    }
    #[test]
    fn test_3() {
        let input = String::from("MCMXCIV");
        let answer = Solution::roman_to_int(input);
        assert_eq!(answer, 1994);
    }

}