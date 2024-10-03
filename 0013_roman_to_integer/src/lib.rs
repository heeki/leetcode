use std::collections::HashMap;

pub struct Solution {}
impl Solution {
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
        for (i,c) in x.chars().enumerate() {
            if x.chars().nth(i).unwrap() == 'I' {
                if i+1 < x.chars().count() && (x.chars().nth(i+1).unwrap() == 'V' || x.chars().nth(i+1).unwrap() == 'X') {
                    result -= map.get(&c).copied().unwrap_or(0);
                } else {
                    result += map.get(&c).copied().unwrap_or(0);
                }
            } else if x.chars().nth(i).unwrap() == 'X' {
                if i+1 < x.chars().count() && (x.chars().nth(i+1).unwrap() == 'L' || x.chars().nth(i+1).unwrap() == 'C') {
                    result -= map.get(&c).copied().unwrap_or(0);
                } else {
                    result += map.get(&c).copied().unwrap_or(0);
                }
            } else if x.chars().nth(i).unwrap() == 'C' {
                if i+1 < x.chars().count() && (x.chars().nth(i+1).unwrap() == 'D' || x.chars().nth(i+1).unwrap() == 'M') {
                    result -= map.get(&c).copied().unwrap_or(0);
                } else {
                    result += map.get(&c).copied().unwrap_or(0);
                }
            } else {
                result += map.get(&c).copied().unwrap_or(0);
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