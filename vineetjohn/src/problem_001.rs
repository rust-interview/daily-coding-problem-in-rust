use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn check_sums(array: Vec<i64>, k: i64) -> bool {
        let mut potential_solutions: HashSet<i64> = HashSet::new();
        for num in array.iter() {
            if potential_solutions.contains(num) {
                return true;
            }
            potential_solutions.insert(k - (*num));
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_sums() {
        assert!(!Solution::check_sums(vec![], 17));
        assert!(Solution::check_sums(vec![10, 15, 3, 7], 17));
        assert!(!Solution::check_sums(vec![1, 2, 3], 17));
    }
}
