use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::matrix::Matrix;

pub struct GaussZeidelModel
{
    pub m: Matrix,
    pub e: f64,
    pub h: f64,
    source: Matrix,
    top: Option<Vec<f64>>,
    bot: Option<Vec<f64>>,
    right: Option<Vec<f64>>,
    left: Option<Vec<f64>>,
}

impl GaussZeidelModel
{
    pub fn new(e: f64, h:f64, nx: usize, ny: usize, top: Option<Vec<f64>>,
               bot: Option<Vec<f64>>, left: Option<Vec<f64>>, right: Option<Vec<f64>>, source: Matrix) -> Self {
        let mut m = Matrix::new(ny, nx);
        if let Some(ref vals) = top {
            assert_eq!(vals.len(), nx);
            for j in 0..nx {
                m[(0,j)] = vals[j];
            }
        }
        if let Some(ref vals) = bot {
            assert_eq!(vals.len(), nx);
            for j in 0..nx {
                m[(ny-1, j)] = vals[j];
            }
        }
        if let Some(ref vals) = left {
            assert_eq!(vals.len(), ny);
            for i in 0..ny {
                m[(i,0)] = vals[i];
            }
        }
        if let Some(ref vals) = right {
            assert_eq!(vals.len(), ny);
            for i in 0..ny {
                m[(i, nx-1)] = vals[i];
            }
        }
        GaussZeidelModel {m:m, e:e, h:h, source:source,
                          top:top, bot:bot, right:right, left:left}
    }
    fn step(&mut self) -> bool {
        let nx = self.m.n_cols;
        let ny = self.m.n_rows;
        let h2 = self.h * self.h;
        let mut sum_new = 0.0;
        let mut sum_old = 0.0;
        for i in 0..ny {
            for j in 0..nx {
                let is_fixed = (i == 0 && self.top.is_some()) ||
                                      (i == ny-1 && self.bot.is_some()) ||
                                      (j == 0 && self.left.is_some()) ||
                                      (j == nx-1 && self.right.is_some());
                if is_fixed {
                    continue;
                }
                let new_val = if i > 0 && i < ny - 1 && j > 0 && j < nx - 1 {
                    0.25 * (
                        self.m[(i-1, j)] + self.m[(i+1, j)] +
                        self.m[(i, j-1)] + self.m[(i, j+1)] -
                        h2 * self.source[(i,j)]
                    )
                } else {
                    let mut sum = 0.0;
                    match j {
                        0 => sum += 2.0 * self.m[(i,1)],
                        j if j == nx-1 => sum += 2.0 * self.m[(i,nx-2)],
                        _ => sum += self.m[(i, j-1)] + self.m[(i, j+1)],
                    };
                    match i {
                        0 => sum += 2.0 * self.m[(1,j)],
                        i if i == ny-1 => sum += 2.0 * self.m[(ny-2, j)],
                        _ => sum += self.m[(i-1, j)] + self.m[(i+1, j)],
                    }
                    (sum - h2 * self.source[(i,j)]) / 4.0
                };
                sum_old += self.m[(i,j)].abs();
                sum_new += new_val.abs();
                self.m[(i,j)] = new_val;
            }
        }
        if (sum_new - sum_old).abs() < self.e {
            return true;
        }
        return false;
    }

    fn OverRelaxation_step(&mut self, r:f64) -> bool {
        let nx = self.m.n_cols;
        let ny = self.m.n_rows;
        let h2 = self.h * self.h;
        let mut sum_control = 0.0;
        for i in 0..ny {
            for j in 0..nx {
                let is_fixed = (i == 0 && self.top.is_some()) ||
                                     (i == ny-1 && self.bot.is_some()) ||
                                     (j == 0 && self.left.is_some()) ||
                                     (j == nx-1 && self.right.is_some());
                if is_fixed {
                    continue;
                }
                let new_val = if i > 0 && i < ny - 1 && j > 0 && j < nx - 1 {
                    r * 0.25 * (
                        self.m[(i-1,j)] + self.m[(i+1,j)] +
                        self.m[(i,j-1)] + self.m[(i,j+1)] -
                        h2 * self.source[(i,j)]
                    ) + (1.0 - r) * self.m[(i,j)]
                } else {
                    let mut sum = 0.0;
                    match j {
                        0 => sum += 2.0 * self.m[(i,1)],
                        j if j == nx-1 => sum += 2.0 * self.m[(i,nx-2)],
                        _ => sum += self.m[(i, j-1)] + self.m[(i,j+1)],
                    }
                    match i {
                        0 => sum += 2.0 * self.m[(1,j)],
                        i if i == ny-1 => sum += 2.0 * self.m[(ny-2,j)],
                        _ => sum += self.m[(i-1, j)] + self.m[(i+1,j)],
                    }
                    r * (sum - h2 * self.source[(i,j)]) * 0.25 + (1.0 - r ) * self.m[(i,j)]
                };
                sum_control += (self.m[(i,j)] - new_val).abs();
                self.m[(i,j)] = new_val;
            }
        }
        if sum_control < self.e {
            return true;
        }
        return false;
    }

    pub fn calculate(&mut self) {
        let mut k: usize = 0;
        let mut is_not_resolved = true;
        while is_not_resolved {
            is_not_resolved = !self.step();
            k = k + 1;
            if (k % 1000) == 0 {println!("Iteration num k = {}", k);}
            if k > 100000 {
                println!("m = \n{}", self.m);
                panic!("too many iterations");
            }
        }
        println!("Iteration num k = {}\nВЫХОД!!!", k);
    }

    pub fn calculate_relax(&mut self, param: f64) {
        let mut k: usize = 0;
        let mut is_not_resolved = true;
        while is_not_resolved {
            is_not_resolved = !self.OverRelaxation_step(param);
            k = k + 1;
            // if (k % 1000) == 0 {println!("Iteration num k = {}", k);}
            if k > 100000 {
                // println!("m = \n{}", self.m);
                eprintln!("too many iterations param = {:.5}", param);
                break;
            }
        }
        if !is_not_resolved {
            println!("Iteration num = {},\t param = {:.5}\nВыход!!!", k, param);
        }
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
        let top = Some(vec![0.0; 10]);
        let bot = Some(vec![0.0; 10]);
        let left = Some(vec![0.0; 10]);
        let right = Some(vec![0.0; 10]);
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

        let top = Some(vec![5.0; 10]);
        let bot = Some(vec![5.0; 10]);
        let left = Some(vec![5.0; 10]);
        let right = Some(vec![5.0; 10]);
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