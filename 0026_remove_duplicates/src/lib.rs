pub struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i: usize = 0;
        while i < nums.len() {
            if i == 0 {
                i += 1;
            } else if nums[i] == nums[i-1] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        // println!("{:?} -> {}", nums, nums.len());
        nums.len() as i32
    }

    pub fn remove_duplicates_builtin(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input: Vec<i32> = vec![1,1,2];
        let answer = Solution::remove_duplicates(&mut input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_2() {
        let mut input: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
        let answer = Solution::remove_duplicates(&mut input);
        assert_eq!(answer, 5);
    }
}