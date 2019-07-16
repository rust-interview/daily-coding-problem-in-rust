#[allow(dead_code)]
pub fn add(a: i32, b: i32) -> i32 {
    // only print when a test fails
    println!("{}", a + b);
    return a + b;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_ne!(add(1, 2), 4);
    }
}
