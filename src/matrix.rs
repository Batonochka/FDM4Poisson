use std::{ops::{Index, IndexMut, Sub}, fmt::{Display}};

use rayon::{iter::{IndexedParallelIterator, ParallelIterator}, slice::ParallelSliceMut};

#[derive(Debug, Clone)]
pub struct Matrix
{
    pub n_cols: usize,
    pub n_rows: usize,
    pub data: Vec<f64>,
}
impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix{data: vec![f64::default(); rows*cols], n_cols: cols, n_rows: rows}
    }
}
impl Index<(usize, usize)> for Matrix
{
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.data[self.n_cols * row + col]
    }
}
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.data[self.n_cols * row + col]
    }    
}
impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        let n_cols = self.n_cols;
        let n_rows = self.n_rows;
        let mut result = Matrix::new(n_rows, n_cols);
        result.data
            .par_chunks_mut(n_cols)
            .enumerate()
            .for_each(|(i, row)| {
                for j in 0..n_cols
                {
                    row[j] = self[(i, j)] - rhs[(i,j)];
                }
            });
        result
    }
}
impl Display for Matrix
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                if j > 0 {
                    write!(f, "\t")?;
                }
                write!(f, "{:.2}", self.data[i*self.n_cols + j])?;
            }
            if i < self.n_rows - 1 {
                write!(f, "\n")?;
            }
        }
        write!(f, "]\n")?;
        Ok(())
    }
}