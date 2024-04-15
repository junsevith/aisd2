use std::time::Instant;
use lista2::{chart, merge, sort};

fn main() {
    let k = 10;
    let nrange = (1000..=50000).step_by(1000);
    let mut rng = rand::thread_rng();
    let mut compdata = Vec::new();
    let mut timedata = Vec::new();

    for n in nrange.clone() {
        println!("n: {}", n);
        let mut comps = (0, 0, 0);
        let mut times = (0, 0, 0);

        for _k in 0..k {
            let mut array = sort::random_array(n, &mut rng);

            let before = Instant::now();
            let result = merge::merge_sort(&array);
            times.0 += before.elapsed().as_nanos();
            comps.0 += result.1;

            let before = Instant::now();
            let result = merge::my_sort(&array);
            times.1 += before.elapsed().as_nanos();
            comps.1 += result.1;

            let before = Instant::now();
            let result = merge::shuffle_sort(&array);
            times.2 += before.elapsed().as_nanos();
            comps.2 += result.1;
        }

        let k = k as f64;
        compdata.push((comps.0 as f64 / k, comps.1 as f64 / k, comps.2 as f64 / k));
        timedata.push((times.0 as f64 / k, times.1 as f64 / k, times.2 as f64 / k));
    }
    let compn = chart::divide_series(compdata.clone(), nrange.clone());
    let timen = chart::divide_series(timedata.clone(), nrange.clone());


    chart::draw_chart(compdata, nrange.clone(), 0.0..800000.0, "Merge Comparisons".to_string());
    chart::draw_chart(compn, nrange.clone(), 0.0..20.0, "Merge Comparisons per n".to_string());
    chart::draw_chart(timedata, nrange.clone(), 0.0..40000000.0, "Merge Time".to_string());
    chart::draw_chart(timen, nrange.clone(), 0.0..2000.0, "Merge Time per n".to_string());
}