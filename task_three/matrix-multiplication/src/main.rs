mod easter_egg;
mod matrix;
use matrix::Matrix;

fn main() -> anyhow::Result<()> {
    // As it was said
    // The program should take two matrices as input
    // (you can hard-code them initially or read them from a file).
    let a = Matrix::new(vec![vec![1, 2], vec![3, 4]])?;

    let b = Matrix::new(vec![vec![5, 6], vec![7, 8]])?;

    let result = a.multiply(&b)?;
    println!("Result:\n{}", result);

    Ok(())
}
