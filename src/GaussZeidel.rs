use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::matrix::Matrix;

pub struct GaussZeidelModel
{
    pub m: Matrix,
    pub e: f64,
    pub h: f64,
    source: Matrix,
}

impl GaussZeidelModel
{
    pub fn new(e: f64, h:f64, nx: usize, ny: usize, top: Vec<f64>,
               bot: Vec<f64>, left: Vec<f64>, right: Vec<f64>, source: Matrix) -> Self {
        let mut m = Matrix::new(ny, nx);
        for j in 0..nx {
            m[(0,j)] = top[j];
            m[(ny-1,j)] = bot[j];
        }
        for i in 0..ny{
            m[(i,0)] = left[i];
            m[(i, nx-1)] = right[i];
        }
        GaussZeidelModel {m:m, e:e, h:h, source:source}
    }
    fn step(&mut self) -> bool {
        let nx = self.m.n_cols;
        let ny = self.m.n_rows;
        let h2 = self.h * self.h;
        let mut counter: usize = 0;
        let inner_num = (nx - 2) * (ny - 2);
        for i in 1..ny-1 {
            for j in 1..nx-1 {
                let new_val = 0.25 * (
                    self.m[(i-1, j)] + self.m[(i+1, j)] +
                    self.m[(i, j-1)] + self.m[(i, j+1)] -
                    h2 * self.source[(i,j)]
                );
                match (new_val - self.m[(i,j)]).abs() < self.e {
                    true => counter = counter + 1,
                    false => counter = 0,
                }
                self.m[(i,j)] = new_val;
            }
        }
        // (1..ny-1).par_iter_mut().for_each(|i| {
        //     for j in 1..nx-1 {
        //         if (i+j) % 2 == 0 {
        //             let new_val = 0.25 * (
        //                 self.m[(i-1, j)] + self.m[(i+1, j)] +
        //                 self.m[(i, j-1)] + self.m[(i, j+1)] - 
        //                 h2 * self.source[(i,j)]
        //             );
        //             match (new_val - self.m[(i,j)]).abs() < self.e {
        //                 true => counter = counter + 1,
        //                 false => counter = 0,
        //             }
        //             self.m[(i,j)] = new_val;
        //         }
        //     }
        // });
        // (1..ny-1).into_par_iter().for_each(|i| {
        //     for j in 1..nx-1 {
        //         if (i+j) % 2 == 1 {
        //             let new_val = 0.25 * (
        //                 self.m[(i-1, j)] + self.m[(i+1, j)] +
        //                 self.m[(i, j-1)] + self.m[(i, j+1)] -
        //                 h2 * self.source[(i,j)]
        //             );
        //             match (new_val - self.m[(i,j)]).abs() < self.e {
        //                 true => counter = counter + 1,
        //                 false => counter = 0,
        //             }
        //             self.m[(i,j)] = new_val;
        //         }
        //     }
        // });
        counter == inner_num
    }

    pub fn calculate(&mut self) {
        let mut k: usize = 0;
        let mut is_not_resolved = true;
        while is_not_resolved {
            is_not_resolved = !self.step();
            k = k + 1;
            if (k % 1000) == 0 {println!("Iteration num k = {}", k);}
            if k > 10000 {
                println!("m = \n{}", self.m);
                panic!("too many iterations");
            }
        }
        println!("Iteration num k = {}\nВЫХОД!!!", k);
    }
}


#[cfg(test)]
mod tests {
    use crate::{GaussZeidel::GaussZeidelModel, matrix::Matrix};


    #[test]
    fn test_zero_solution()
    {
        let nx: usize = 10;
        let ny: usize = 10;
        let h = 1.0 / (nx as f64);
        let e = 1e-8;
        let top = vec![0.0; 10];
        let bot = vec![0.0; 10];
        let left = vec![0.0; 10];
        let right = vec![0.0; 10];
        let source = Matrix::new(ny, nx);

        let mut solver = GaussZeidelModel::new(e, h, nx, ny, top, bot, left, right, source);
        solver.calculate();
        for i in 1..ny-1 {
            for j in 1..nx {
                assert!(solver.m[(i,j)].abs() < 1e-5);
            }
        }
    }

    #[test]
    fn eqvipotencial_test()
    {
        let nx: usize = 10;
        let ny: usize = 10;
        let h = 1.0 / (nx as f64);
        let e = 1e-8;

        let top = vec![5.0; 10];
        let bot = vec![5.0; 10];
        let left = vec![5.0; 10];
        let right = vec![5.0; 10];
        let source = Matrix::new(ny, nx);

        let mut solver = GaussZeidelModel::new(e, h, nx, ny, top, bot, left, right, source);
        solver.calculate();
        for i in 1..ny-1 {
            for j in 1..nx-1 {
                assert!((solver.m[(i,j)] - 5.0).abs() < 1e-5);
            }
        }
    }
}