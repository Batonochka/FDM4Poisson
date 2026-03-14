use std::vec;

use crate::{GaussZeidel::GaussZeidelModel, jacobi::JacobiModel, matrix::Matrix, visual::plot_wireframe};

mod matrix;
mod jacobi;
mod visual;
mod GaussZeidel;
mod OverRelaxation;
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


    // вообще надо бы на нормальные блоки разбить все, чтоб было хоть немного понятно что и как робит
    // но мне похер)
    let nx: usize = 100;
    let ny: usize = 100;
    let e = 0.001;
    let h = 0.1;
    let top = Some(vec![100.0; 100]);
    let bot = Some(vec![-100.0; 100]);
    // let mut right = Some(vec![0.0; 100]);
    // let mut left = Some(vec![0.0; 100]);
    let right = None;
    let left = None;
    let source = Matrix::new(ny, nx);
    // if let Some(ref mut vec) = right {
    //     vec[0] = 100.0;
    // }
    // if let Some(ref mut vec ) = left {
    //     vec[0] = 100.0
    // }

    // let mut model = JacobiModel::new(e, h, nx, ny, top, bot, left, right, source);
    let mut model = GaussZeidelModel::new(e, h, nx, ny, top, bot, left, right, source);
    // println!("m_cur = \n{}\n", model.m_cur);
    model.calculate();

    // model.calculate_relax(1.5);

    // let filename = "firts.html";
    let filename = "Condensator.html";
    // let filename = "relax_Gauss.html";
    // plot_wireframe(&model.m_cur, &h, Some(filename));
    plot_wireframe(&model.m, &h, Some(filename));

    // let h = 0.5;
    // let n = 5;
    // let x: Vec<f64> = (0..n).map(|i| i as f64 * h).collect();
    // println!("{:?}", x);
}
