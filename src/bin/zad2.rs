use std::ops::Range;
use plotters::prelude::*;
use crate::sort;

pub fn simulation(){
    let k = 10;
    let nrange = (1000..=50000).step_by(1000);
    let mut rng = rand::thread_rng();
    let mut swapdata = Vec::new();
    let mut compdata = Vec::new();

    for n in nrange.clone() {
        println!("n: {}", n);
        let mut comps = (0, 0, 0);
        let mut swaps = (0, 0, 0);

        for _k in 0..k {
            let mut array = sort::random_array(n, &mut rng);
            // let result = insertion_sort(&mut array.clone());
            // comps.0 += result.0;
            // swaps.0 += result.1;

            let result = sort::quick_sort(&mut array.clone());
            comps.1 += result.0;
            swaps.1 += result.1;

            let result = sort::hybrid_sort(&mut array.clone(), 3);
            comps.2 += result.0;
            swaps.2 += result.1;
        }

        let k = k as f64;
        compdata.push((comps.0 as f64 / k, comps.1 as f64 / k, comps.2 as f64 / k));
        swapdata.push((swaps.0 as f64 / k, swaps.1 as f64 / k, swaps.2 as f64 / k));
    }
    let compn = divide_series(compdata.clone(), nrange.clone());

    let swapn = divide_series(swapdata.clone(), nrange.clone());


    draw_chart(compdata, nrange.clone(), 0.0..1000000.0, "Comparisons".to_string());
    draw_chart(swapdata, nrange.clone(), 0.0..200000.0, "Swaps".to_string());
    draw_chart(compn, nrange.clone(), 0.0..30.0, "Comparisons per n".to_string());
    draw_chart(swapn, nrange.clone(), 0.0..5.0, "Swaps per n".to_string());
}

fn divide_series(data: Vec<(f64, f64, f64)>, n_range: impl Iterator<Item=usize>) -> Vec<(f64, f64, f64)> {
    n_range.zip(data.iter()).map(|x| {
        let n = x.0 as f64;
        let data = x.1;
        (data.0 / n, data.1 / n, data.2 / n)
    }).collect()
}

fn draw_chart(data: Vec<(f64, f64, f64)>, mut n_range: impl Iterator<Item=usize> + Clone, y_range: Range<f64>, name: String) {
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
        .y_label_area_size(40)
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