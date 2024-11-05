/// Note: This is a joke implementation to show how it could be done with a prod-ready library.
/// You know, it's better to stay on the shoulders of giants.
/// Initially I thought about just using the `ndarray` crate,
/// but then I decided that I want this job more than I want to be funny.
use ndarray::Array2;

#[allow(dead_code)]
pub fn multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Option<Vec<Vec<i32>>> {
    if a.is_empty() || b.is_empty() || a[0].is_empty() || b[0].is_empty() || a[0].len() != b.len() {
        return None;
    }

    let a_array =
        Array2::from_shape_vec((a.len(), a[0].len()), a.iter().flatten().cloned().collect())
            .ok()?;

    let b_array =
        Array2::from_shape_vec((b.len(), b[0].len()), b.iter().flatten().cloned().collect())
            .ok()?;

    let result = a_array.dot(&b_array);

    Some(result.rows().into_iter().map(|row| row.to_vec()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_multiplication() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![5, 6], vec![7, 8]];

        let result = multiply(&a, &b).unwrap();
        assert_eq!(result, vec![vec![19, 22], vec![43, 50]]);
    }
}
