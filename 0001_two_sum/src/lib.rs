pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for i in 0..nums.len() {
            let diff: i32 = target - nums[i];
            if i < nums.len()-1 {
                if let Some(j) = nums[i+1..nums.len()].iter().position(|&x| x == diff) {
                    println!("target={}, nums[{}]={} diff={} nums[{}]={}", target, i, nums[i], diff, i+j+1, nums[i+j+1]);
                    result.push(i as i32);
                    result.push((i+j+1) as i32);
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input: Vec<i32> = vec![2,7,11,15];
        let target: i32 = 9;
        let validation = Solution::two_sum(input, target);
        assert_eq!(validation, vec![0,1])
    }

    #[test]
    fn example_2() {
        let input: Vec<i32> = vec![3,2,4];
        let target: i32 = 6;
        let validation = Solution::two_sum(input, target);
        assert_eq!(validation, vec![1,2])
    }

    #[test]
    fn example_3() {
        let input: Vec<i32> = vec![3,3];
        let target: i32 = 6;
        let validation = Solution::two_sum(input, target);
        assert_eq!(validation, vec![0,1])
    }
}