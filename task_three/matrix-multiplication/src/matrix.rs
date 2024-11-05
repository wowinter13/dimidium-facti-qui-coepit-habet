use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatrixError {
    #[error("Invalid matrix dimensions: {0}")]
    InvalidDimensions(String),
    #[error("Matrix multiplication dimensions mismatch: {0}")]
    DimensionMismatch(String),
    #[error("Thread error: {0}")]
    ThreadError(String),
    #[error("Arithmetic overflow")]
    Overflow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(data: Vec<Vec<i32>>) -> Result<Self, MatrixError> {
        if data.is_empty() {
            return Err(MatrixError::InvalidDimensions(
                "Matrix cannot be empty".to_string(),
            ));
        }

        let rows = data.len();
        let cols = data[0].len();

        if data.iter().any(|row| row.len() != cols) {
            return Err(MatrixError::InvalidDimensions(
                "All rows must have the same length".to_string(),
            ));
        }

        Ok(Matrix { data, rows, cols })
    }

    pub fn multiply(&self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.cols != other.rows {
            return Err(MatrixError::DimensionMismatch(format!(
                "Cannot multiply {}x{} matrix with {}x{} matrix",
                self.rows, self.cols, other.rows, other.cols
            )));
        }

        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let chunk_size = (self.rows + num_threads - 1) / num_threads;
        let result = Arc::new(Mutex::new(vec![vec![0; other.cols]; self.rows]));
        let mut handles = vec![];

        let self_data = self.data.clone();
        let other_data = other.data.clone();

        for chunk_start in (0..self.rows).step_by(chunk_size) {
            let chunk_end = (chunk_start + chunk_size).min(self.rows);
            let result = Arc::clone(&result);
            let self_data = self_data.clone();
            let other_data = other_data.clone();

            let handle = thread::spawn(move || {
                for i in chunk_start..chunk_end {
                    for j in 0..other_data[0].len() {
                        let mut sum: i64 = 0; // i64 to prevent overflow
                        for (k, row) in other_data.iter().enumerate().take(self_data[0].len()) {
                            sum = sum
                                .checked_add(
                                    (self_data[i][k] as i64)
                                        .checked_mul(row[j] as i64)
                                        .ok_or(MatrixError::Overflow)?,
                                )
                                .ok_or(MatrixError::Overflow)?;
                        }
                        // Convert back to i32 if within range
                        let result_value = i32::try_from(sum).map_err(|_| MatrixError::Overflow)?;

                        result
                            .lock()
                            .map_err(|e| MatrixError::ThreadError(e.to_string()))?[i][j] =
                            result_value;
                    }
                }
                Ok::<(), MatrixError>(())
            });

            handles.push(handle);
        }

        for handle in handles {
            handle
                .join()
                .map_err(|_| MatrixError::ThreadError("Thread panicked".to_string()))??;
        }

        let result_data = Arc::try_unwrap(result)
            .map_err(|_| MatrixError::ThreadError("Failed to unwrap result".to_string()))?
            .into_inner()
            .map_err(|e| MatrixError::ThreadError(e.to_string()))?;

        Matrix::new(result_data)
    }

    #[cfg(test)]
    pub fn get(&self, row: usize, col: usize) -> Option<i32> {
        self.data.get(row)?.get(col).copied()
    }
}
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() -> Result<(), MatrixError> {
        let matrix = Matrix::new(vec![vec![1, 2], vec![3, 4]])?;
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.get(0, 0), Some(1));
        assert_eq!(matrix.get(1, 1), Some(4));
        Ok(())
    }

    #[test]
    fn test_empty_matrix() {
        let result = Matrix::new(vec![]);
        assert!(matches!(result, Err(MatrixError::InvalidDimensions(_))));
    }

    #[test]
    fn test_invalid_dimensions() {
        let result = Matrix::new(vec![vec![1, 2], vec![3]]);
        assert!(matches!(result, Err(MatrixError::InvalidDimensions(_))));
    }

    #[test]
    fn test_basic_multiplication() -> Result<(), MatrixError> {
        let a = Matrix::new(vec![vec![1, 2], vec![3, 4]])?;

        let b = Matrix::new(vec![vec![5, 6], vec![7, 8]])?;

        let result = a.multiply(&b)?;

        assert_eq!(result.get(0, 0), Some(19)); // 1*5 + 2*7
        assert_eq!(result.get(0, 1), Some(22)); // 1*6 + 2*8
        assert_eq!(result.get(1, 0), Some(43)); // 3*5 + 4*7
        assert_eq!(result.get(1, 1), Some(50)); // 3*6 + 4*8
        Ok(())
    }

    #[test]
    fn test_dimension_mismatch() -> Result<(), MatrixError> {
        let a = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]])?;

        let b = Matrix::new(vec![vec![7, 8], vec![9, 10]])?;

        let result = a.multiply(&b);
        assert!(matches!(result, Err(MatrixError::DimensionMismatch(_))));
        Ok(())
    }

    #[test]
    fn test_large_matrix_multiplication() -> Result<(), MatrixError> {
        let size: usize = 50;
        let a = Matrix::new(
            (0..size)
                .map(|i| (0..size).map(|j| ((i + j) % 10) as i32).collect())
                .collect(),
        )?;

        let b = Matrix::new(
            (0..size)
                .map(|i| (0..size).map(|j| ((i + j) % 10) as i32).collect())
                .collect(),
        )?;

        let result = a.multiply(&b)?;
        assert_eq!(result.rows, size);
        assert_eq!(result.cols, size);

        let first_element = result.get(0, 0).unwrap();
        assert!(first_element >= 0);
        assert!(first_element < size as i32 * 10 * 10);

        Ok(())
    }

    #[test]
    fn test_multiply_identity() -> Result<(), MatrixError> {
        let a = Matrix::new(vec![vec![1, 2], vec![3, 4]])?;

        let identity = Matrix::new(vec![vec![1, 0], vec![0, 1]])?;

        let result = a.multiply(&identity)?;
        assert_eq!(result.data, a.data);
        Ok(())
    }
}
