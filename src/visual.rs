use plotly::{Layout, Plot, Surface, common::Title, contour::Contours, layout::{self, Axis, LayoutScene}};

use crate::matrix::Matrix;



pub fn plot_wireframe(data: &Matrix, h: &f64, filename: Option<&str>)
{
    let nx = data.n_cols;
    let ny = data.n_rows;
    let x: Vec<f64> = (0..ny).map(|i| i as f64 * (*h)).collect();
    let y: Vec<f64> = (0..nx).map(|j| j as f64 * (*h)).collect();

    let z: Vec<Vec<f64>> = (0..ny).map(|i| 
                           (0..nx).map(|j| data[(i,j)]).collect())
                           .collect();
    

    let trace = Surface::new(z).x(x).y(y);

    let layout = Layout::new()
        .title("3D поверхность потенциала") // так работает
        .scene(
            LayoutScene::new()
                .x_axis(Axis::new().title("x"))
                .y_axis(Axis::new().title("y"))
                .z_axis(Axis::new().title("U").range(vec![-100.0, 100.0])),
        );

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);

    if let Some(name) = filename {
        std::fs::write(name, plot.to_html()).expect("Не удалось записать файл");
    } else {
        plot.show();
    }
}