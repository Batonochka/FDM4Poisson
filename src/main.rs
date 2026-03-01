use crate::{jacobi::JacobiModel, matrix::Matrix, visual::plot_wireframe};

mod matrix;
mod jacobi;
mod visual;
fn main() {
    let nx = 10;
    let ny = 10;
    let e = 0.001;
    let h = 0.1;
    let top = vec![5.0; 10];
    let bot = vec![5.0; 10];
    let left = vec![5.0; 10];
    let right = vec![5.0; 10];
    let source = Matrix::new(ny, nx);

    let mut model = JacobiModel::new(e, h, nx, ny, top, bot, left, right, source);
    println!("m_cur = \n{}\n", model.m_cur);
    println!("m_old = \n{}",model.m_old);
    println!("<----------!!!!--------->\n");
    model.calculate();
    println!("m_cur = \n{}\n", model.m_cur);
    println!("m_old = \n{}",model.m_old);

    let filename = "hueta.html";
    plot_wireframe(&model.m_cur, &model.h, Some(filename));

    // let h = 0.5;
    // let n = 5;
    // let x: Vec<f64> = (0..n).map(|i| i as f64 * h).collect();
    // println!("{:?}", x);
}
