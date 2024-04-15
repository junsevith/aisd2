use std::ops::Range;
use plotters::prelude::*;

pub fn divide_series(data: Vec<(f64, f64, f64)>, n_range: impl Iterator<Item=usize>) -> Vec<(f64, f64, f64)> {
    n_range.zip(data.iter()).map(|x| {
        let n = x.0 as f64;
        let data = x.1;
        (data.0 / n, data.1 / n, data.2 / n)
    }).collect()
}

pub fn draw_chart(data: Vec<(f64, f64, f64)>, mut n_range: impl Iterator<Item=usize> + Clone, y_range: Range<f64>, name: String) {
    let file = format!("charts/chart_{}.png", name);

    let mut n_range_copy = n_range.clone();
    let first = n_range_copy.next().unwrap() as f64;
    let last = n_range_copy.last().unwrap() as f64;
    let x_range = first..last;

    let mut drawing_area = BitMapBackend::new(&file, (800, 600)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&drawing_area)
        .caption(name, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range, y_range)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();


    ctx.draw_series(LineSeries::new(n_range.clone().zip(data.clone().iter()).map(|x| (x.0 as f64, x.1.0)), &RED)).unwrap();
    ctx.draw_series(LineSeries::new(n_range.clone().zip(data.clone().iter()).map(|x| (x.0 as f64, x.1.1)), &GREEN)).unwrap();
    ctx.draw_series(LineSeries::new(n_range.zip(data.iter()).map(|x| (x.0 as f64, x.1.2)), &BLUE)).unwrap();


    // let mut data_iter = data.iter();
    //
    //
    // for x in n_range {
    //     let x = x as f64;
    //     let current = data_iter.next().unwrap();
    //     ctx.draw_series(iter::once(Circle::new((x, current.0), 2, RED.filled()))).unwrap();
    //     ctx.draw_series(iter::once(Circle::new((x, current.1), 2, GREEN.filled()))).unwrap();
    //     ctx.draw_series(iter::once(Circle::new((x, current.2), 2, BLUE.filled()))).unwrap();
    // }
}

pub fn divide_log(data: Vec<(f64, f64, f64)>, n_range: impl Iterator<Item=usize>) -> Vec<(f64, f64, f64)> {
    n_range.zip(data.iter()).map(|x| {
        let n = x.0 as f64;
        let n = n*n.ln();
        let data = x.1;
        (data.0 / n, data.1 / n, data.2 / n)
    }).collect()
}