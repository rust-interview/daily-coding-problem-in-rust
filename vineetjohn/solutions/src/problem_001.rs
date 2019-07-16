#[allow(dead_code)]
pub fn check_sums(array: i32,  k: i32) -> i32 {
    return array + k
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_check_sums() {
        assert_eq!(check_sums(1, 2), 3);
    }
}