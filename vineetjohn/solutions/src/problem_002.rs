#[allow(dead_code)]
pub fn get_factors(array: Vec<i64>) -> Vec<i64> {
    let mut cumulative_product: i64 = 1;
    let mut right_prod_array: Vec<i64> = Vec::new();
    for num in array.iter() {
        cumulative_product *= num;
        right_prod_array.push(cumulative_product);
    }

    cumulative_product = 1;
    let mut left_prod_array: Vec<i64> = Vec::new();
    let mut rev = array.clone();
    rev.reverse();
    for num in rev.iter() {
        cumulative_product *= num;
        left_prod_array.push(cumulative_product);
    }
    left_prod_array.reverse();

    let mut output_array: Vec<i64> = Vec::new();
    for i in 0..array.len() {
        let num: i64;
        if i == 0 {
            num = left_prod_array[i + 1];
        } else if i == (array.len() - 1) {
            num = right_prod_array[i - 1];
        } else {
            num = right_prod_array[i - 1] * left_prod_array[i + 1]
        }
        output_array.push(num);
    }

    output_array
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_factors() {
        assert_eq!(get_factors(vec![1, 2, 3, 4, 5]), vec![120, 60, 40, 30, 24]);
        assert_eq!(get_factors(vec![3, 2, 1]), vec![2, 3, 6]);
    }
}
