use math::round;

pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    fn get_digit_count(x: &i32) -> i32 {
        let f = *x as f64;
        (round::floor(f.log10(), 0) as i32) + 1
    }

    #[allow(dead_code)]
    fn is_palindrome_i32(x: i32) -> bool {
        let digits = Solution::get_digit_count(&x);
        println!("x={}, digits={}", x, digits);
        true
    }

    #[allow(dead_code)]
    fn is_palindrome_str(x: i32) -> bool {
        let converted: String = x.to_string();
        let reversed: String = converted.chars().rev().collect();
        let halfway = converted.len()/2 + 1;
        let mut response = true;
        for i in 0..halfway {
            // println!("converted[{}]={}, reversed[{}]={}", i, converted.chars().nth(i).unwrap(), i, reversed.chars().nth(i).unwrap());
            let evaluation = converted.chars().nth(i).unwrap() == reversed.chars().nth(i).unwrap();
            response = response && evaluation;
        }
        // println!("converted={}, reversed={}, halfway={}, response={}", converted, reversed, halfway, response);
        response
    }

    pub fn is_palindrome(x: i32) -> bool {
        Solution::is_palindrome_str(x)
        // Solution::is_palindrome_i32(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = 121;
        let answer = Solution::is_palindrome(input);
        assert_eq!(answer, true);
    }

    #[test]
    fn test_2() {
        let input = -121;
        let answer = Solution::is_palindrome(input);
        assert_eq!(answer, false);
    }

    #[test]
    fn test_3() {
        let input = 10;
        let answer = Solution::is_palindrome(input);
        assert_eq!(answer, false);
    }
}
