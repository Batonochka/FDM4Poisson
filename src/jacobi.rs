use crate::matrix::Matrix;
use rayon::prelude::*;

pub struct JacobiModel {
    pub m_old: Matrix,
    pub m_cur: Matrix,
    pub e: f64,
    pub h: f64,
    source: Matrix,
}

impl JacobiModel {
    pub fn new(e:f64, h:f64, nx:usize, ny:usize, top:Vec<f64>, bot:Vec<f64>,
    left:Vec<f64>, right:Vec<f64>, source: Matrix) -> Self {
        let mut m_cur = Matrix::new(ny, nx);
        for j in 0..nx {
            m_cur[(0, j)] = top[j];
            m_cur[(ny-1, j)] = bot[j];
        }
        for i in 0..ny {
            m_cur[(i, 0)] = left[i];
            m_cur[(i, nx-1)] = right[i];
        }
        JacobiModel{
            m_old:Matrix::new(ny, nx),
            m_cur:m_cur,
            e: e,
            h: h,
            source: source,
        }
    }

    fn step(&mut self) {
        let nx = self.source.n_cols;
        let ny = self.source.n_rows;
        let h = self.h;

        self.m_old.data.copy_from_slice(&self.m_cur.data); // через указатели переделать пиграться с памятью (указатели)
        let m_old = &self.m_old;
        let source = &self.source;
        self.m_cur.data
            .par_chunks_mut(nx)
            .enumerate()
            .for_each(|(i, row)| {
                if i != 0 && i != ny - 1 {
                    for j in 0..nx {
                        if j != 0 && j != nx-1 {
                            row[j] = 0.25 * (m_old[(i,j-1)] + m_old[(i,j+1)] +
                                             m_old[(i-1,j)] + m_old[(i+1,j)] -
                                             h * h * source[(i,j)]);
                        } else {
                            row[j] = m_old[(i, j)];
                        }
                    }
                } else {
                    for j in 0..nx {
                        row[j] = m_old[(i,j)];
                    }
                }
            });
    }

    fn _norm(&self) -> bool {   // переписать норма - сумма модулей всех элементов
        let dif_matrix = &self.m_cur - &self.m_old;
        let n = self.m_cur.n_cols * self.m_cur.n_rows;
        let mut sum_new = 0.0;
        let mut sum_old = 0.0;
        for i in 0..n
        {
            sum_new = sum_new + self.m_cur.data[i];
            sum_old = sum_old + self.m_old.data[i];
            if dif_matrix.data[i].abs() > self.e {return true;}
        }
        if (sum_new - sum_old).abs() > self.e {return true;}
        return false;
    }

    pub fn calculate(&mut self){
        let mut k: usize = 0;
        while self._norm() {
            self.step();
            k = k + 1;
            if k % 1000 == 0 {println!("iteartion num = {}", k)}
            if k > 100000 {
                println!("{}", self.m_cur);
                panic!("too many iterations")
            }
        }
        println!("ВЫХОД!!!! k = {}", k);
    }
}

#[cfg(test)]
mod tests {
    use crate::{jacobi::JacobiModel, matrix::Matrix};

    #[test]
    fn test_zero_solution()
    {
        let nx = 10;
        let ny = 10;
        let h = 1.0 / (nx as f64);
        let e = 1e-8;

        let top = vec![0.0; 10];
        let bot = vec![0.0; 10];
        let left = vec![0.0; 10];
        let right = vec![0.0; 10];
        let source = Matrix::new(ny, nx);

        let mut solver = JacobiModel::new(e, h, nx, ny, top, bot, left, right, source);
        solver.calculate();
        for i in 1..ny-1 {
            for j in 1..nx-1 {
                assert!(solver.m_cur[(i,j)].abs() < 1e-5);
            }
        }
    }

    #[test]
    fn eqvipotencial_test()
    {
        let nx = 10;
        let ny = 10;
        let h = 1.0 / (nx as f64);
        let e = 1e-8;

        let top = vec![5.0; 10];
        let bot = vec![5.0; 10];
        let left = vec![5.0; 10];
        let right = vec![5.0; 10];
        let source = Matrix::new(ny, nx);

        let mut solver = JacobiModel::new(e, h, nx, ny, top, bot, left, right, source);
        solver.calculate();
        for i in 1..ny-1 {
            for j in 1..nx-1 {
                assert!((solver.m_cur[(i,j)] - 5.0).abs() < 1e-5);
            }
        }
    }
}