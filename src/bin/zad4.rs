use std::time::Instant;

use lista2::{chart, double_pivot, sort};

fn main() {
    let k = 10;
    let nrange = (1000..=50000).step_by(1000);
    let mut rng = rand::thread_rng();
    let mut compdata = Vec::new();
    let mut swapsdata = Vec::new();
    let mut timedata = Vec::new();

    for n in nrange.clone() {
        println!("n: {}", n);
        let mut comps = (0, 0, 0);
        let mut swaps = (0, 0, 0);
        let mut times = (0, 0, 0);

        for _k in 0..k {
            let mut array = sort::random_array(n, &mut rng);
            let mut array2 = array.clone();

            let before = Instant::now();
            let result = sort::quick_sort(&mut array);
            times.0 += before.elapsed().as_nanos();
            comps.0 += result.0;
            swaps.0 += result.1;

            let before = Instant::now();
            let result = double_pivot::dual_pivot_quicksort(&mut array2);
            times.1 += before.elapsed().as_nanos();
            comps.1 += result.0;
            swaps.1 += result.1;

        }

        let k = k as f64;
        compdata.push((comps.0 as f64 / k, comps.1 as f64 / k, comps.2 as f64 / k));
        swapsdata.push((swaps.0 as f64 / k, swaps.1 as f64 / k, swaps.2 as f64 / k));
        timedata.push((times.0 as f64 / k, times.1 as f64 / k, times.2 as f64 / k));
    }
    let compn = chart::divide_log(compdata.clone(), nrange.clone());
    let swapsn = chart::divide_log(swapsdata.clone(), nrange.clone());
    let timen = chart::divide_log(timedata.clone(), nrange.clone());


    chart::draw_chart(compdata, nrange.clone(), 0.0..1200000.0, "Quick Comparisons".to_string());
    chart::draw_chart(compn, nrange.clone(), 0.0..3.0, "Quick Comparisons per nlogn".to_string());
    chart::draw_chart(swapsdata, nrange.clone(), 0.0..400000.0, "Quick Swaps".to_string());
    chart::draw_chart(swapsn, nrange.clone(), 0.0..1.0, "Quick Swaps per nlogn".to_string());
    chart::draw_chart(timedata, nrange.clone(), 0.0..10000000.0, "Quick Time".to_string());
    chart::draw_chart(timen, nrange.clone(), 0.0..20.0, "Quick Time per nlogn".to_string());
}