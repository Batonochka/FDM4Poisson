use std::vec;

use crate::{GaussZeidel::GaussZeidelModel, jacobi::JacobiModel, matrix::Matrix, visual::plot_wireframe};

mod matrix;
mod jacobi;
mod visual;
mod GaussZeidel;
fn main() {
    // let nx = 10;
    // let ny = 10;
    // let e = 0.001;
    // let h = 0.1;
    // let top = vec![5.0; 10];
    // let bot = vec![5.0; 10];
    // let left = vec![5.0; 10];
    // let right = vec![5.0; 10];
    // let source = Matrix::new(ny, nx);

    // let mut model = JacobiModel::new(e, h, nx, ny, top, bot, left, right, source);
    // println!("m_cur = \n{}\n", model.m_cur);
    // println!("m_old = \n{}",model.m_old);
    // println!("<----------!!!!--------->\n");
    // model.calculate();
    // println!("m_cur = \n{}\n", model.m_cur);
    // println!("m_old = \n{}",model.m_old);

    // let filename = "hueta.html";
    // plot_wireframe(&model.m_cur, &model.h, Some(filename)); // тут все работает для метода Якоби


    // let nx = 10;
    // let ny = 10;
    // let e = 0.001;
    // let h = 0.1;
    // let top = vec![5.0; 10];
    // let bot = vec![5.0; 10];
    // let right = vec![5.0; 10];
    // let left = vec![5.0; 10];
    // let source = Matrix::new(ny, nx);

    // let mut model = GaussZeidelModel::new(e, h, nx, ny, top, bot, left, right, source);
    // println!("m beginnin = \n{}\n", model.m);
    // println!("<-----!!!----->");
    // model.calculate();
    // println!("m final = \n{}", model.m);

    // let filename = "huetax2.html";
    // plot_wireframe(&model.m, &model.h, Some(filename));

    let nx: usize = 100;
    let ny: usize = 100;
    let e = 0.001;
    let h = 0.1;
    let top = vec![100.0; 100];
    let bot = vec![0.0; 100];
    let mut right = vec![0.0; 100];
    let mut left = vec![0.0; 100];
    let source = Matrix::new(ny, nx);
    right[0] = 100.0;
    left[0] = 100.0;

    // let mut model = JacobiModel::new(e, h, nx, ny, top, bot, left, right, source);
    let mut model = GaussZeidelModel::new(e, h, nx, ny, top, bot, left, right, source);
    // println!("m_cur = \n{}\n", model.m_cur);
    model.calculate();
    // let filename = "firts.html";
    let filename = "first_Gauss.html";
    // plot_wireframe(&model.m_cur, &h, Some(filename));
    plot_wireframe(&model.m, &h, Some(filename));

    // let h = 0.5;
    // let n = 5;
    // let x: Vec<f64> = (0..n).map(|i| i as f64 * h).collect();
    // println!("{:?}", x);
}
